mod helpers;
pub mod buckets;
pub mod stores;

pub use self::buckets::*;
pub use self::stores::*;

use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;

pub const DEFAULT_BUCKETS_COUNT: usize = 256;

pub trait Bucket<K: Key, V: Value>: Sized + Clone {
    fn new() -> Self;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&self, key: K) -> Option<&V>;
    fn get_mut(&mut self, key: K) -> Option<&mut V>;
    fn reached_max_capacity(&self) -> bool;
}

pub trait BucketStoreNew<K: Key, V: Value, B: Bucket<K, V>>: BucketStore<K, V, B> {
    fn initialized() -> Self;
    fn initialized_with_capacity(cap: usize) -> Self;
}

pub trait BucketStore<K: Key, V: Value, B: Bucket<K, V>> {
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> &B;
    fn get_mut(&mut self, idx: usize) -> &mut B;
    // TODO: Add default implementation that uses get & get_mut
    fn search<P: Fn(&B) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&B>;
    fn search_mut<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B>;
}
