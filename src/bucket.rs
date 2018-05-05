use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;

pub const DEFAULT_BUCKETS_COUNT: usize = 256;

pub trait Bucket<K: Key, V: Value>: Sized + Clone {
    fn empty() -> Self;
    fn push(&mut self, key: K, value: V);
    fn get(&self, key: K) -> Option<&V>;
    fn reached_max_capacity(&self) -> bool;
}

pub type OptionBucket<K, V> = Option<(K, V)>;

impl<K: Key, V: Value> Bucket<K, V> for OptionBucket<K, V> {
    fn empty() -> Self {
        Option::default()
    }

    fn push(&mut self, key: K, value: V) {
        *self = Some((key, value))
    }

    fn get(&self, _key: K) -> Option<&V> {
        self.as_ref().map(|(_, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        self.is_some()
    }
}

pub type SmallVecBucket<K, V> = SmallVec<[(K, V); 1]>;

impl<K: Key, V: Value> Bucket<K, V> for SmallVecBucket<K, V> {
    fn empty() -> Self {
        SmallVec::default()
    }

    fn push(&mut self, key: K, value: V) {
        self.push((key, value))
    }

    fn get(&self, key: K) -> Option<&V> {
        self.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        false
    }
}

pub type VecBucket<K, V> = Vec<(K, V)>;

impl<K: Key, V: Value> Bucket<K, V> for VecBucket<K, V> {
    fn empty() -> Self {
        Vec::with_capacity(1)
    }

    fn push(&mut self, key: K, value: V) {
        self.push((key, value))
    }

    fn get(&self, key: K) -> Option<&V> {
        self.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        false
    }
}

pub trait BucketListNew<K: Key, V: Value, B: Bucket<K, V>>: BucketList<K, V, B> {
    fn initialized() -> Self;
    fn initialized_with_capacity(cap: usize) -> Self;
}

pub trait BucketList<K: Key, V: Value, B: Bucket<K, V>> {
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> &B;
    fn get_mut(&mut self, idx: usize) -> &mut B;
    // TODO: Add default implementation that uses get & get_mut
    fn search<P: Fn(&B) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&B>;
    fn search_mut<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B>;
}

impl<K: Key, V: Value, B: Bucket<K, V>> BucketList<K, V, B> for [B] {
    #[inline]
    fn len(&self) -> usize {
        <[B]>::len(self)
    }

    #[inline]
    fn get(&self, idx: usize) -> &B {
        &self[idx]
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut B {
        &mut self[idx]
    }

    fn search<P: Fn(&B) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&B> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            let bucket = unsafe { self.get_unchecked(i) };

            if predicate(bucket) {
                return Some(bucket);
            }
        }

        None
    }

    fn search_mut<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            // T_T
            // borrowck was mad about the mutable version of search,
            // so I was forced to rewrite it this awful way
            if predicate(unsafe { self.get_unchecked_mut(i) }) {
                return Some(unsafe { self.get_unchecked_mut(i) });
            }
        }

        None
    }
}

impl<T, K: Key, V: Value, B: Bucket<K, V>> BucketList<K, V, B> for T
where
    T: AsRef<[B]> + AsMut<[B]>,
{
    #[inline]
    fn len(&self) -> usize {
        BucketList::<K, V, B>::len(self.as_ref())
    }

    #[inline]
    fn get(&self, idx: usize) -> &B {
        BucketList::<K, V, B>::get(self.as_ref(), idx)
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut B {
        BucketList::<K, V, B>::get_mut(self.as_mut(), idx)
    }

    #[inline]
    fn search<P: Fn(&B) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&B> {
        self.as_ref().search(start_idx, predicate)
    }

    #[inline]
    fn search_mut<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B> {
        self.as_mut().search_mut(start_idx, predicate)
    }
}

impl<K, V, B> BucketListNew<K, V, B> for Vec<B>
where
    K: Key,
    V: Value,
    B: Bucket<K, V>,
{
    fn initialized() -> Self {
        Self::initialized_with_capacity(DEFAULT_BUCKETS_COUNT)
    }

    fn initialized_with_capacity(cap: usize) -> Self {
        use std::iter;
        iter::repeat(B::empty()).take(cap).collect()
    }
}

use std::mem;

macro_rules! impl_bucket_list_for_array {
    ($name:ident, $size:expr) => {
        pub struct $name<K: Key, V: Value, B: Bucket<K, V>> {
            inner: [B; $size],
            _marker: PhantomData<(K, V)>,
        }

        impl<K: Key, V: Value, B: Bucket<K, V>> BucketListNew<K, V, B> for $name<K, V, B> {
            fn initialized() -> Self {
                unsafe {
                    let mut array: [B; $size] = mem::uninitialized();

                    let aptr = array[..].as_mut_ptr();
                    for i in 0..$size {
                        aptr.offset(i).write(B::empty())
                    }

                    $name {
                        inner: array,
                        _marker: PhantomData,
                    }
                }
            }

            fn initialized_with_capacity(_: usize) -> Self {
                panic!("{} is static on-stack structure, thus it has no capacity configureble in runtime. `initialized` should be used instead.", stringify!($name));
            }
        }

        impl<K, V, B> AsRef<[B]> for $name<K, V, B>
        where
            K: Key,
            V: Value,
            B: Bucket<K, V>,
        {
            #[inline]
            fn as_ref(&self) -> &[B] {
                &self.inner[..]
            }
        }

        impl<K, V, B> AsMut<[B]> for $name<K, V, B>
        where
            K: Key,
            V: Value,
            B: Bucket<K, V>,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [B] {
                &mut self.inner[..]
            }
        }
    };
}

impl_bucket_list_for_array!(Array2, 2);
impl_bucket_list_for_array!(Array4, 4);
impl_bucket_list_for_array!(Array8, 8);
impl_bucket_list_for_array!(Array16, 16);
impl_bucket_list_for_array!(Array32, 32);
impl_bucket_list_for_array!(Array64, 64);
impl_bucket_list_for_array!(Array128, 128);
impl_bucket_list_for_array!(Array256, 256);
impl_bucket_list_for_array!(Array512, 512);
impl_bucket_list_for_array!(Array768, 768);
impl_bucket_list_for_array!(Array1024, 1024);
impl_bucket_list_for_array!(Array2048, 2048);
impl_bucket_list_for_array!(Array4096, 4096);
impl_bucket_list_for_array!(Array8192, 8192);
impl_bucket_list_for_array!(Array16384, 16384);
impl_bucket_list_for_array!(Array32768, 32768);

struct WrappingIndexIterator {
    start: usize,
    length: usize,
    current: usize,
    first: bool,
}

impl WrappingIndexIterator {
    pub fn new(start: usize, length: usize) -> Self {
        assert!(start <= length);
        WrappingIndexIterator {
            start,
            length,
            current: start,
            first: true,
        }
    }
}

impl Iterator for WrappingIndexIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // Check that we haven't returned to the beginning
        if self.current != self.start || self.first {
            self.first = false;

            // Check that we shouldn't wrap
            if self.current < self.length {
                let item = self.current;
                self.current += 1;
                Some(item)
            } else {
                self.current = 0;
                self.next()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_index_iterator() {
        let start = 0;
        let length = 5;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[0, 1, 2, 3, 4]);

        let start = 2;
        let length = 5;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[2, 3, 4, 0, 1]);

        let start = 0;
        let length = 1;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[0]);

        let start = 0;
        let length = 0;
        let result = WrappingIndexIterator::new(start, length)
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(&result[..], &[]);
    }

    #[test]
    #[should_panic]
    fn wrapping_index_iterator_invalid_input() {
        let start = 1;
        let length = 0;
        WrappingIndexIterator::new(start, length);
    }

    #[test]
    fn vec_bucket_list_with_flat_bucket() {
        let bl = vec![Some((1, 1)), Some((2, 2))];
        let bucket = bl.search(1, |bucket| bucket.map(|(k, _v)| k == 1).unwrap_or(false));
        assert_eq!(bucket.unwrap().unwrap(), (1, 1));
    }

    #[test]
    fn vec_bucket_list_with_vec_bucket() {
        let bl = vec![vec![(1, 1), (2, 2)], vec![(3, 3)]];
        let bucket = bl.search(1, |bucket| bucket.iter().any(|(k, v)| *k == 2));
        assert_eq!(bucket.unwrap(), &vec![(1, 1), (2, 2)]);
    }
}
