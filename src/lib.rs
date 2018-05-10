#![feature(nll)]
extern crate smallvec;
extern crate unreachable;

pub mod bucket;
pub mod hash;
pub mod kv;

use std::marker::PhantomData;

pub use bucket::{Bucket, BucketStore, BucketStoreNew, OptionBucket, SmallVecBucket2,
                 SmallVecBucket1, SmallVecBucket4, VecBucket};
pub use hash::{DefaultHasher, Hash, Hasher};
pub use kv::{Key, Value};

pub struct PrimitiveMap<
    K: Key,
    V: Value,
    B: Bucket<K, V> + 'static = SmallVecBucket4<K, V>,
    BL: BucketStore<K, V, B> = Vec<B>,
    H: Hasher<K> = DefaultHasher<K>,
> {
    buckets: BL,
    _marker: PhantomData<(K, V, H, B)>,
}

impl<K, V, B, BL, H> Clone for PrimitiveMap<K, V, B, BL, H>
where
    K: Key,
    V: Value,
    B: Bucket<K, V> + Clone,
    BL: BucketStore<K, V, B> + Clone,
    H: Hasher<K> + Default,
{
    fn clone(&self) -> Self {
        PrimitiveMap::custom(self.buckets.clone(), H::default())
    }
}

/// `Vec`-based `map` with `SmallVec`(4) buckets.
/// The balanced default
pub type VecPrimitiveMap<K, V> =
    PrimitiveMap<K, V, SmallVecBucket4<K, V>, Vec<SmallVecBucket4<K, V>>, DefaultHasher<K>>;

/// `Array`-based `map` with `SmallVec`(2) buckets.
/// The main array is stored on the stack,
/// the buckets may extend onto heap.
pub type ArrayPrimitiveMap<K, V, A> =
    PrimitiveMap<K, V, SmallVecBucket1<K, V>, A, DefaultHasher<K>>;

/// Linear-probing PrimitiveMap alias.
/// Useful in embedded environments and where on-stack `map` alignment is necessary
pub type LinearPrimitiveMap<K, V, A> = PrimitiveMap<K, V, OptionBucket<K, V>, A, DefaultHasher<K>>;

impl<K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key,
    V: Value,
    B: Bucket<K, V>,
    BL: BucketStore<K, V, B> + BucketStoreNew<K, V, B>,
    H: Hasher<K>,
{
    pub fn new() -> Self {
        PrimitiveMap::custom(BL::initialized(), H::default())
    }

    pub fn with_capacity(cap: usize) -> Self {
        PrimitiveMap::custom(BL::initialized_with_capacity(cap), H::default())
    }
}

impl<K, V, B, BL> PrimitiveMap<K, V, B, BL>
where
    K: Key,
    V: Value,
    B: Bucket<K, V>,
    BL: BucketStore<K, V, B>,
    DefaultHasher<K>: Hasher<K>,
{
    pub fn with_buckets(buckets: BL) -> Self {
        PrimitiveMap::custom(buckets, DefaultHasher::new())
    }
}

impl<K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key,
    V: Value,
    B: Bucket<K, V>,
    BL: BucketStore<K, V, B>,
    H: Hasher<K>,
{
    pub fn custom(buckets: BL, _: H) -> Self {
        PrimitiveMap {
            buckets,
            _marker: PhantomData,
        }
    }
}

impl<K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key,
    V: Value,
    B: Bucket<K, V>,
    BL: BucketStore<K, V, B>,
    H: Hasher<K>,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let addr = self.get_addr(key);
        let bucket = self.buckets
            .search_bucket(addr, |bucket| !bucket.reached_max_capacity())
            .expect("PrimitiveMap capacity is exhausted");
        bucket.insert(key, value)
    }

    #[inline]
    pub fn get(&self, key: K) -> Option<&V> {
        self.get_key_value(key).map(|(_, v)| v)
    }

    #[inline]
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.get_key_value_mut(key).map(|(_, v)| v)
    }

    #[inline]
    pub fn get_key_value(&self, key: K) -> Option<(K, &V)> {
        let addr = self.get_addr(key);
        self.buckets.search_entry(addr, key)
    }

    #[inline]
    pub fn get_key_value_mut(&mut self, key: K) -> Option<(K, &mut V)> {
        let addr = self.get_addr(key);
        self.buckets.search_entry_mut(addr, key)
    }

    #[inline]
    pub fn remove(&mut self, key: K) -> Option<V> {
        self.get_bucket(key).and_then(|b| b.remove(key))
    }

    #[inline]
    pub fn remove_entry(&mut self, key: K) -> Option<(K, V)> {
        self.get_bucket(key).and_then(|b| b.remove_entry(key))
    }

    #[inline]
    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buckets.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buckets.clear()
    }

    #[inline]
    fn get_bucket(&mut self, key: K) -> Option<&mut B> {
        let addr = self.get_addr(key);
        self.buckets.search_bucket(addr, |b| b.get(key).is_some())
    }

    #[inline]
    fn get_addr(&self, key: K) -> usize {
        let hash = H::hash(key);
        H::compress(hash, self.buckets.len())
    }
}

impl<'a, K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key + 'a,
    V: Value + 'a,
    B: Bucket<K, V> + 'a,
    BL: BucketStore<K, V, B> + 'a,
    H: Hasher<K>,
    &'a BL: IntoIterator<Item = &'a B>,
    &'a B: IntoIterator<Item = &'a (K, V)>,
{
    pub fn buckets(&'a self) -> impl Iterator<Item = &'a B> + 'a {
        self.buckets.into_iter()
    }

    pub fn keys(&'a self) -> impl Iterator<Item = K> + 'a {
        self.buckets()
            .flat_map(|b| b.into_iter().map(|entry| entry.0))
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a (K, V)> + 'a {
        self.buckets().flat_map(|b| b.into_iter())
    }

    pub fn values(&'a self) -> impl Iterator<Item = &'a V> + 'a {
        self.buckets().flat_map(|b| b.into_iter()).map(|(_, v)| v)
    }
}

impl<'a, K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key + 'a,
    V: Value + 'a,
    B: Bucket<K, V> + 'a,
    BL: BucketStore<K, V, B> + 'a,
    H: Hasher<K>,
    &'a mut BL: IntoIterator<Item = &'a mut B>,
    &'a mut B: IntoIterator<Item = &'a mut (K, V)>,
{
    pub fn buckets_mut(&'a mut self) -> impl Iterator<Item = &'a mut B> + 'a {
        self.buckets.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)> + 'a {
        self.buckets_mut().flat_map(|b| b.into_iter())
    }

    pub fn values_mut(&'a mut self) -> impl Iterator<Item = &'a mut V> + 'a {
        self.buckets_mut()
            .flat_map(|b| b.into_iter())
            .map(|(_, v)| v)
    }
}

impl<K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key,
    V: Value,
    B: Bucket<K, V> + IntoIterator<Item = (K, V)>,
    BL: BucketStore<K, V, B> + IntoIterator<Item = B>,
    H: Hasher<K>,
{
    pub fn into_iter(self) -> impl Iterator<Item = (K, V)> {
        self.buckets.into_iter().flat_map(|b| b.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bucket::{Array1024, Array64};

    #[test]
    fn create_vec() {
        // Vec map with StackVec(1) bucket
        let mut map = VecPrimitiveMap::new();
        map.insert(1, 1);
        map.get(1);
    }

    #[test]
    fn create_lp() {
        // Array64 map with Option<(K, V)> buckets (linear probing)
        let mut map = LinearPrimitiveMap::with_buckets(Array64::initialized());
        map.insert(1, 1);
        map.get(1);
    }

    #[test]
    fn create_custom() {
        let buckets = Vec::<OptionBucket<_, _>>::initialized_with_capacity(1000);
        let hasher = DefaultHasher::new();
        let mut map = PrimitiveMap::custom(buckets, hasher);
        map.insert(1, 1);
        map.get(1);
    }

    #[test]
    fn insert_dynamic() {
        let mut map = VecPrimitiveMap::new();
        map.insert(0u8, 10u32);
    }

    #[test]
    fn insert_linear() {
        let mut map = LinearPrimitiveMap::with_buckets(Array64::initialized());
        map.insert(0u16, 10u32);
    }

    #[test]
    fn get_empty_dynamic() {
        let map = VecPrimitiveMap::new();
        assert_eq!(map.get(0u32), None::<&u32>);
    }

    #[test]
    fn get_empty_linear() {
        let map = LinearPrimitiveMap::with_buckets(Array64::initialized());
        assert_eq!(map.get(0u32), None::<&u32>);
    }

    #[test]
    fn insert_and_get_vec() {
        let mut map = VecPrimitiveMap::new();
        map.insert(0i8, 10u32);
        assert_eq!(map.get(0i8), Some(&10u32));
    }

    #[test]
    fn insert_and_get_linear() {
        let mut map = LinearPrimitiveMap::with_buckets(Array64::initialized());
        map.insert(0i16, 10u32);
        assert_eq!(map.get(0i16), Some(&10u32));
    }

    #[test]
    fn insert_saturate_buckets_vec() {
        let mut map = VecPrimitiveMap::with_capacity(100);
        for i in 0..10000 {
            map.insert(i, i);
        }
        for i in 0..10000 {
            assert_eq!(map.get(i), Some(&i))
        }
    }

    #[test]
    fn insert_full_load_linear_probing() {
        let mut map = LinearPrimitiveMap::with_buckets(Array1024::initialized());
        for i in 0..1024 {
            map.insert(i, i);
        }
        for i in 0..1024 {
            assert_eq!(map.get(i), Some(&i))
        }
    }

    #[test]
    fn iter_vec_map() {
        let mut map = VecPrimitiveMap::with_capacity(100);
        let mut space = vec![1, 2, 3, 4];
        for i in &space {
            map.insert(*i, *i);
        }

        let map_space = map.iter().collect::<Vec<_>>();

        assert_eq!(map_space.len(), space.len());

        for i in 0..space.len() {
            let entry = map_space[i];
            assert_eq!(space[i], entry.0);
            assert_eq!(space[i], entry.1);
        }
    }
}
