extern crate smallvec;

pub mod bucket;
pub mod hash;
pub mod kv;

use std::marker::PhantomData;

pub use bucket::{Bucket, BucketList};
pub use hash::{DefaultHasher, Hash, Hasher};
pub use kv::{Key, Value};
/*
pub struct PrimitiveMap<
    K: Key,
    V: Value,
    B: Bucket<K, V>,
    BL: BucketList<K, V, Bucket = B>,
    H: Hasher<K>,
> {
    buckets: BL,
    _marker: PhantomData<(K, V, B, H)>,
}

impl<K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
where
    K: Key,
    V: Value,
    B: Bucket<K, V>,
    BL: BucketList<K, V, Bucket = B>,
    H: Hasher<K>,
{
    pub fn custom(buckets: BL, _hasher: H) -> Self {
        PrimitiveMap {
            buckets,
            _marker: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let addr = self.get_addr(key);
        let bucket = self.buckets.get_mut(addr);
        bucket.push(key, value)
    }

    pub fn get(&self, key: K) -> Option<V> {
        let addr = self.get_addr(key);
        let bucket = &self.buckets.get(addr);
        bucket.get(key)
    }

    // Linear probing
    pub fn find_bucket<P: Fn(&B) -> bool>(&self, start_idx: addr, predicate: P) -> Option<&B> {
        let idx = start_idx;

        loop {
            // Get bucket from the list
            let bucket = self.buckets.get(idx);

            // Test the bucket against predicate
            if predicate(bucket) {
                return Some(bucket)
            }

            // Increment index
            idx = (idx + 1) % self.buckets.len();

            // On the full cycle, return None
            if idx == start_idx {
                return None
            }
        }
    }

    fn get_addr(&self, key: K) -> usize {
        let hash = H::hash(key);
        H::compress(hash, self.buckets.len())
    }
}

impl<K, V> PrimitiveMap<K, V, SmallVecBucket<K, V>, SmallVecBucketList<K, V>, DefaultHasher<K>>
where
    K: Key,
    V: Value,
    DefaultHasher<K>: Hasher<K>,
{
    pub fn dynamic() -> Self {
        PrimitiveMap::custom(SmallVecBucketList::empty(), DefaultHasher::default())
    }
}

impl<K, V> PrimitiveMap<K, V, ArrayBucket<K, V>, ArrayBucketList<K, V>, DefaultHasher<K>>
where
    K: Key,
    V: Value,
    DefaultHasher<K>: Hasher<K>,
{
    pub fn fixed() -> Self {
        PrimitiveMap::custom(ArrayBucketList::empty(), DefaultHasher::default())
    }
}

impl<K, V, H> PrimitiveMap<K, V, SmallVecBucket<K, V>, SmallVecBucketList<K, V>, H>
where
    K: Key,
    V: Value,
    H: Hasher<K>,
{
    pub fn dynamic_with_hasher(hasher: H) -> Self {
        PrimitiveMap::custom(SmallVecBucketList::empty(), hasher)
    }
}

impl<K, V, H> PrimitiveMap<K, V, ArrayBucket<K, V>, ArrayBucketList<K, V>, H>
where
    K: Key,
    V: Value,
    H: Hasher<K>,
{
    pub fn fixed_with_hasher(hasher: H) -> Self {
        PrimitiveMap::custom(ArrayBucketList::empty(), hasher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dynamic() {
        <PrimitiveMap<u8, u32, _, _, _>>::dynamic();
    }

    #[test]
    fn create_fixed() {
        <PrimitiveMap<u8, u32, _, _, _>>::fixed();
    }

    #[test]
    fn default_hasher() {
        <PrimitiveMap<u8, u8, _, _, _>>::dynamic();
        <PrimitiveMap<u16, u8, _, _, _>>::dynamic();
        <PrimitiveMap<u32, u8, _, _, _>>::dynamic();
        <PrimitiveMap<u64, u8, _, _, _>>::dynamic();
        <PrimitiveMap<usize, u8, _, _, _>>::dynamic();
        <PrimitiveMap<i8, u8, _, _, _>>::dynamic();
        <PrimitiveMap<i16, u8, _, _, _>>::dynamic();
        <PrimitiveMap<i32, u8, _, _, _>>::dynamic();
    }

    #[test]
    fn insert_dynamic() {
        let mut map = PrimitiveMap::dynamic();
        map.insert(0u8, 10u32);
    }

    #[test]
    fn insert_fixed() {
        let mut map: PrimitiveMap<_, _, _, _, DefaultHasher<_>> = PrimitiveMap::fixed();
        map.insert(0u16, 10u32);
    }

    #[test]
    fn get_empty_dynamic() {
        let map = PrimitiveMap::dynamic();
        assert_eq!(map.get(0u32), None::<u32>);
    }

    #[test]
    fn get_empty_fixed() {
        let map = PrimitiveMap::fixed();
        assert_eq!(map.get(0u32), None::<u32>);
    }

    #[test]
    fn insert_and_get_dynamic() {
        let mut map = PrimitiveMap::dynamic();
        map.insert(0i8, 10u32);
        assert_eq!(map.get(0i8), Some(10u32));
    }

    #[test]
    fn insert_and_get_fixed() {
        let mut map = PrimitiveMap::fixed();
        map.insert(0i16, 10u32);
        assert_eq!(map.get(0i16), Some(10u32));
    }

    #[test]
    fn insert_saturate_buckets_dynamic() {
        let mut map = PrimitiveMap::dynamic();
        for i in 0..10000 {
            map.insert(i, 10u32);
        }
    }
}
*/