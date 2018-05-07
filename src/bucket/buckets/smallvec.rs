use bucket::Bucket;
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;

use bucket::helpers::IndexOf;

macro_rules! impl_bucket_for_small_vec {
    ($name:ident, $size:expr) => {
        pub type $name<K, V> = SmallVec<[(K, V); $size]>;
        impl<K: Key, V: Value> Bucket<K, V> for $name<K, V> {
            #[inline]
            fn new() -> Self {
                SmallVec::default()
            }

            #[inline]
            fn insert(&mut self, key: K, value: V) -> Option<V> {
                let old_value = self.index_of(key).map(|idx| self.swap_remove(idx).1);
                self.push((key, value));
                old_value
            }

            #[inline]
            fn get(&self, key: K) -> Option<&V> {
                self.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
            }

            #[inline]
            fn get_mut(&mut self, key: K) -> Option<&mut V> {
                self.iter_mut().find(|(k, _)| *k == key).map(|(_, v)| v)
            }

            #[inline]
            fn reached_max_capacity(&self) -> bool {
                false
            }
        }

        impl<K: Key, V: Value> IndexOf<K> for $name<K, V> {
            #[inline]
            fn index_of(&self, key: K) -> Option<usize> {
                self.iter()
                    .enumerate()
                    .find(|(_, (k, _))| *k == key)
                    .map(|(idx, _)| idx)
            }
        }
    };
}

impl_bucket_for_small_vec!(SmallVecBucket1, 1);
impl_bucket_for_small_vec!(SmallVecBucket2, 2);
impl_bucket_for_small_vec!(SmallVecBucket4, 4);
impl_bucket_for_small_vec!(SmallVecBucket8, 8);
impl_bucket_for_small_vec!(SmallVecBucket16, 16);
impl_bucket_for_small_vec!(SmallVecBucket32, 32);
impl_bucket_for_small_vec!(SmallVecBucket64, 64);
