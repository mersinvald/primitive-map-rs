pub mod buckets;
mod helpers;
pub mod stores;

pub use self::buckets::*;
pub use self::stores::*;

use bucket::helpers::WrappingIndexIterator;
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;

pub const DEFAULT_BUCKETS_COUNT: usize = 256;

pub trait Bucket<K: Key, V: Value>: Sized + Clone {
    fn new() -> Self;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn get<'a>(&'a self, key: K) -> Option<&'a V>;

    fn get_mut(&mut self, key: K) -> Option<&mut V>;

    fn remove(&mut self, key: K) -> Option<V> {
        self.remove_entry(key).map(|entry| entry.1)
    }

    fn remove_entry(&mut self, key: K) -> Option<(K, V)>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self);

    fn reached_max_capacity(&self) -> bool;
}

pub trait BucketStoreNew<K: Key, V: Value, B: Bucket<K, V> + 'static>: BucketStore<K, V, B> {
    fn initialized() -> Self;
    fn initialized_with_capacity(cap: usize) -> Self;
}

pub trait BucketStore<K: Key, V: Value, B: Bucket<K, V> + 'static> {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get<'a>(&'a self, idx: usize) -> &'a B;

    fn get_mut(&mut self, idx: usize) -> &mut B;

    fn search_bucket<P: Fn(&B) -> bool>(
        &mut self,
        start_idx: usize,
        predicate: P,
    ) -> Option<&mut B> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            let bucket = self.get_mut(i);
            if predicate(bucket) {
                return Some(bucket);
            }
        }
        None
    }

    fn search_entry(&self, start_idx: usize, key: K) -> Option<(K, &V)> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            if let Some(value) = self.get(i).get(key) {
                return Some((key, value));
            }
        }
        None
    }

    fn search_entry_mut(&mut self, start_idx: usize, key: K) -> Option<(K, &mut V)> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            let mut bucket = self.get_mut(i);
            if let Some(value) = bucket.get_mut(key) {
                return Some((key, value));
            }
        }
        None
    }

    fn clear(&mut self) {
        for i in 0..self.len() {
            self.get_mut(i).clear()
        }
    }
}
