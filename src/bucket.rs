use std::usize;
use kv::{Key, Value};
use smallvec::SmallVec;

pub trait Bucket<K: Key, V: Value>: Sized + Clone {
    fn new() -> Self;
    fn push(&mut self, key: K, value: V);
    fn get(&self, key: K) -> Option<&V>;
    fn reached_max_capacity(&self) -> bool;
}

impl<K: Key, V: Value> Bucket<K, V> for Option<(K, V)> {
    fn new() -> Self {
        Option::default()
    }

    fn push(&mut self, key: K, value: V) {
        *self = Some((key, value))
    }

    fn get(&self, _key: K) -> Option<&V> {
        self.as_ref().map(|(k, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        self.is_some()
    }
}

impl<K: Key, V: Value> Bucket<K, V> for SmallVec<[(K, V); 1]> {
    fn new() -> Self {
        SmallVec::default()
    }

    fn push(&mut self, key: K, value: V) {
        self.push((key, value))
    }

    fn get(&self, key: K) -> Option<&V> {
        self.iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        false
    }
}

impl<K: Key, V: Value> Bucket<K, V> for Vec<(K, V)> {
    fn new() -> Self {
        Vec::with_capacity(1)
    }

    fn push(&mut self, key: K, value: V) {
        self.push((key, value))
    }

    fn get(&self, key: K) -> Option<&V> {
        self.iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        false
    }
}

pub trait BucketListNew<K: Key, V: Value> {
    type Bucket: Bucket<K, V>;
    fn new(len: Option<usize>) -> Self;
}

pub trait BucketList<K: Key, V: Value> {
    type Bucket: Bucket<K, V>;
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> &Self::Bucket;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket;
    // TODO: Add default implementation that uses get & get_mut
    fn search<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&Self::Bucket>;
    fn search_mut<P: Fn(&Self::Bucket) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut Self::Bucket>;
}

impl<K: Key, V: Value, B: Bucket<K, V> + 'static> BucketList<K, V> for [B] {
    type Bucket = B;

    fn len(&self) -> usize {
        <[B]>::len(self)
    }

    fn get(&self, idx: usize) -> &Self::Bucket {
        &self[idx]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket {
        &mut self[idx]
    }

    fn search<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&Self::Bucket> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            let bucket = unsafe {
                self.get_unchecked(i)
            };

            if predicate(bucket) {
                return Some(bucket)
            }
        }

        None
    }

    fn search_mut<P: Fn(&Self::Bucket) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut Self::Bucket> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            // T_T
            // borrowck was mad about the mutable version of search,
            // so I was forced to rewrite it this awful way
            if predicate(unsafe {self.get_unchecked_mut(i)}) {
                return Some(unsafe { self.get_unchecked_mut(i) })
            }
        }

        None
    }
}

use std::ops::{Deref,  DerefMut};

impl<T, K: Key, V: Value, B: Bucket<K, V> + 'static> BucketList<K, V> for T
    where T: Deref<Target=[B]> + DerefMut
{
    type Bucket = B;

    fn len(&self) -> usize {
        <[B] as BucketList<K, V>>::len(self)
    }

    fn get(&self, idx: usize) -> &Self::Bucket {
        <[B] as BucketList<K, V>>::get(self, idx)
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket {
        <[B] as BucketList<K, V>>::get_mut(self, idx)
    }

    fn search<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&Self::Bucket> {
        <[B] as BucketList<K, V>>::search(self, start_idx, predicate)
    }

    fn search_mut<P: Fn(&Self::Bucket) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut Self::Bucket> {
        <[B] as BucketList<K, V>>::search_mut(self, start_idx, predicate)
    }
}

impl<K: Key, V: Value, B: Bucket<K, V>> BucketListNew<K, V> for Vec<B> {
    type Bucket = B;

    fn new(len: Option<usize>) -> Self {
        use std::iter;
        let len = len.expect("Vec bucket list should be provided with length");
        iter::repeat(B::new()).take(len).collect()
    }
}

use std::mem;

macro_rules! impl_bucket_list_for_array {
    ($size:expr) => {
        impl<K: Key, V: Value, B: Bucket<K, V>> BucketListNew<K, V> for [B; $size] {
            type Bucket = B;

            fn new(_: Option<usize>) -> Self {
                unsafe {
                    let mut array: [B; $size] = mem::uninitialized();
                    let aptr = array[..].as_mut_ptr();
                    for i in 0..$size {
                        aptr.offset(i).write(B::new())
                    }
                    array
                }
            }
        }
    };
}

impl_bucket_list_for_array!(2);
impl_bucket_list_for_array!(4);
impl_bucket_list_for_array!(8);
impl_bucket_list_for_array!(16);
impl_bucket_list_for_array!(32);
impl_bucket_list_for_array!(64);
impl_bucket_list_for_array!(128);
impl_bucket_list_for_array!(256);
impl_bucket_list_for_array!(512);
impl_bucket_list_for_array!(768);
impl_bucket_list_for_array!(1024);
impl_bucket_list_for_array!(2048);
impl_bucket_list_for_array!(4096);
impl_bucket_list_for_array!(8192);
impl_bucket_list_for_array!(16384);
impl_bucket_list_for_array!(32768);

struct WrappingIndexIterator {
    start: usize,
    length: usize,
    current: usize,
    first: bool
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
        let result = WrappingIndexIterator::new(start, length).into_iter().collect::<Vec<_>>();
        assert_eq!(&result[..], &[0, 1, 2, 3, 4]);

        let start = 2;
        let length = 5;
        let result = WrappingIndexIterator::new(start, length).into_iter().collect::<Vec<_>>();
        assert_eq!(&result[..], &[2, 3, 4, 0, 1]);

        let start = 0;
        let length = 1;
        let result = WrappingIndexIterator::new(start, length).into_iter().collect::<Vec<_>>();
        assert_eq!(&result[..], &[0]);

        let start = 0;
        let length = 0;
        let result = WrappingIndexIterator::new(start, length).into_iter().collect::<Vec<_>>();
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
        let bucket = bl.search(1, |bucket| bucket.map(|(k, v)| k == 1).unwrap_or(false));
        assert_eq!(bucket.unwrap().unwrap(), (1, 1));
    }

    #[test]
    fn vec_bucket_list_with_vec_bucket() {
        let bl = vec![vec![(1, 1), (2, 2)], vec![(3, 3)]];
        let bucket = bl.search(1, |bucket| {
            bucket.iter().any(|(k, v)| *k == 2)
        });
        assert_eq!(bucket.unwrap(), &vec![(1, 1), (2, 2)]);
    }
}
