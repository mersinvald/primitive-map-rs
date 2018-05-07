use bucket::Bucket;
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;

pub type OptionBucket<K, V> = Option<(K, V)>;

impl<K: Key, V: Value> Bucket<K, V> for OptionBucket<K, V> {
    fn new() -> Self {
        Option::default()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let old_value = self.take().map(|(_, v)| v);
        *self = Some((key, value));
        old_value
    }

    fn get(&self, _key: K) -> Option<&V> {
        self.as_ref().map(|(_, v)| v)
    }

    fn get_mut(&mut self, _key: K) -> Option<&mut V> {
        self.as_mut().map(|(_, v)| v)
    }

    fn reached_max_capacity(&self) -> bool {
        self.is_some()
    }
}
