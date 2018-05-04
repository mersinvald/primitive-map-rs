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

pub trait BucketList<K: Key, V: Value> {
    type Bucket: Bucket<K, V>;
    fn new(len: Option<usize>) -> Self;
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> &Self::Bucket;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket;
    // TODO: Add default implementation that uses get & get_mut
    fn search<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&Self::Bucket>;
    fn search_mut<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&mut Self::Bucket>;
}

impl<K: Key, V: Value, B: Bucket<K, V>> BucketList<K, V> for Vec<B> {
    type Bucket = B;

    fn new(len: Option<usize>) -> Self {
        use std::iter;
        let len = len.expect("Vec bucket list should be provided with length");
        iter::repeat(B::new()).take(len).collect()
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, idx: usize) -> &Self::Bucket {
        &self[idx]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket {
        &mut self[idx]
    }

    fn search<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&Self::Bucket> {
        assert!(start_idx < self.len())

    }
    fn search_mut<P: Fn(&Self::Bucket) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&mut Self::Bucket>;
}

use std::mem;

macro_rules! impl_bucket_list_for_array {
    ($size:expr) => {
        impl<K: Key, V: Value, B: Bucket<K, V>> BucketList<K, V> for [B; $size] {
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

            fn len(&self) -> usize {
                $size
            }

            fn get(&self, idx: usize) -> &Self::Bucket {
                &self[idx]
            }

            fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket {
                &mut self[idx]
            }
        }
    };
}

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
}

impl Iterator for WrappingIndexIterator {

}

fn wrapping_index_iterator(start: usize, length: usize) -> impl Iterator<Item=usize> {
    start
}