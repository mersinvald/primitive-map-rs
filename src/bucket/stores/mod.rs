pub mod vec;
pub mod array;

pub use self::array::*;

use bucket::{Bucket, BucketStore};
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;
use bucket::helpers::WrappingIndexIterator;


impl<K: Key, V: Value, B: Bucket<K, V> + 'static> BucketStore<K, V, B> for [B] {
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

    #[inline]
    fn search_bucket<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            let bucket = unsafe { self.get_unchecked_mut(i) };
            if predicate(bucket) {
                return Some(bucket);
            }
        }

        None
    }


    #[inline]
    fn search_entry(&self, start_idx: usize, key: K) -> Option<(K, &V)> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            let bucket = unsafe { self.get_unchecked(i) };
            if let Some(value) = bucket.get(key) {
                return Some((key, value))
            }
        }

        None
    }

    #[inline]
    fn search_entry_mut(&mut self, start_idx: usize, key: K) -> Option<(K, &mut V)> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            let bucket = unsafe {self.get_unchecked_mut(i)};
            if let Some(value) = bucket.get_mut(key) {
                return Some((key, value))
            }
        }
        None
    }
}


impl<T, K: Key, V: Value, B: Bucket<K, V> + 'static> BucketStore<K, V, B> for T
    where
        T: AsRef<[B]> + AsMut<[B]>,
{
    #[inline]
    fn len(&self) -> usize {
        BucketStore::<K, V, B>::len(self.as_ref())
    }

    #[inline]
    fn get(&self, idx: usize) -> &B {
        BucketStore::<K, V, B>::get(self.as_ref(), idx)
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut B {
        BucketStore::<K, V, B>::get_mut(self.as_mut(), idx)
    }

    #[inline]
    fn search_bucket<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B> {
        self.as_mut().search_bucket(start_idx, predicate)
    }

    #[inline]
    fn search_entry(&self, start_idx: usize, key: K) -> Option<(K, &V)> {
        self.as_ref().search_entry(start_idx, key)
    }

    #[inline]
    fn search_entry_mut(&mut self, start_idx: usize, key: K) -> Option<(K, &mut V)> {
        self.as_mut().search_entry_mut(start_idx, key)
    }
}
