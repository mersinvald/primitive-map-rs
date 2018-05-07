use std::collections::BTreeMap;
use bucket::Bucket;
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;
use std::mem;

use bucket::helpers::IndexOf;

pub type BTreeBucket<K, V> = BTreeMap<K, V>;

impl<K: Key, V: Value> Bucket<K, V> for BTreeBucket<K, V> {
    #[inline]
    fn new() -> Self {
        BTreeMap::new()
    }

    #[inline]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    #[inline]
    fn get(&self, key: K) -> Option<&V> {
        self.get(&key)
    }

    #[inline]
    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.get_mut(&key)
    }

    #[inline]
    fn reached_max_capacity(&self) -> bool {
        false
    }

    #[inline]
    fn len(&self) -> usize {
        BTreeBucket::len(self)
    }

    fn remove_entry(&mut self, key: K) -> Option<(K, V)> {
        self.remove(&key).map(|v| (key, v))
    }

    fn clear(&mut self) {
        BTreeMap::clear(self)
    }
}
