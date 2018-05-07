use bucket::Bucket;
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;
use std::mem;

pub type OptionBucket<K, V> = Option<(K, V)>;

impl<K: Key, V: Value> Bucket<K, V> for OptionBucket<K, V> {
    #[inline]
    fn new() -> Self {
        Option::default()
    }

    #[inline]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut new_entry = Some((key, value));
        mem::swap(self, &mut new_entry);
        new_entry.map(|(_, v)| v)
    }

    #[inline]
    fn get(&self, _key: K) -> Option<&V> {
        self.as_ref().map(|(_, v)| v)
    }

    #[inline]
    fn get_mut(&mut self, _key: K) -> Option<&mut V> {
        self.as_mut().map(|(_, v)| v)
    }

    #[inline]
    fn reached_max_capacity(&self) -> bool {
        self.is_some()
    }
}
