#![feature(test)]
extern crate test;
extern crate indexmap;

extern crate smallvec;

pub mod bucket;
pub mod kv;
pub mod hash;
mod bench;

use std::marker::PhantomData;
use std::mem;
use smallvec::SmallVec;

use bucket::{
    Bucket,
    BucketList,
    SmallVecBucket,
    SmallVecBucketList,
    ArrayBucket,
    ArrayBucketList,
};

use kv::{Key, Value};

use hash::{
    Hasher, Hash, DefaultHasher,
};

struct PrimitiveMap<K: Key, V: Value, B: Bucket<K, V>, BL: BucketList<K, V, Bucket=B>, H: Hasher<K> = DefaultHasher<K>> {
    buckets: BL,
    _marker: PhantomData<(K, V, B, H)>,
}

impl<K, V, B, BL, H> PrimitiveMap<K, V, B, BL, H>
    where K: Key,
          V: Value,
          B: Bucket<K, V>,
          BL: BucketList<K, V, Bucket=B>,
          H: Hasher<K>,
{
    fn with_bucket_list(buckets: BL) -> Self {
        PrimitiveMap {
            buckets,
            _marker: PhantomData,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let addr = self.get_addr(key);
        let bucket = self.buckets.get_mut(addr);
        bucket.push(key, value)
    }

    fn get(&self, key: K) -> Option<V> {
        let addr = self.get_addr(key);
        let bucket = &self.buckets.get(addr);
        bucket.get(key)
    }

    fn get_addr(&self, key: K) -> usize {
        let hash = H::hash(key);
        H::compress(hash, self.buckets.len())
    }
}

impl<K, V, H> PrimitiveMap<K, V, SmallVecBucket<K, V>, SmallVecBucketList<K, V>, H>
    where K: Key,
          V: Value,
          H: Hasher<K>,
{
    fn with_dynamic_bucket() -> Self {
        PrimitiveMap {
            buckets: SmallVecBucketList::empty(),
            _marker: PhantomData,
        }
    }
}

impl<K, V, H> PrimitiveMap<K, V, ArrayBucket<K, V>, ArrayBucketList<K, V>, H>
    where K: Key,
          V: Value,
          H: Hasher<K>,
{
    fn with_fixed_bucket() -> Self {
        PrimitiveMap {
            buckets: ArrayBucketList::empty(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dynamic() {
        let map: PrimitiveMap<u64, u8, _, _> = PrimitiveMap::with_dynamic_bucket();
    }

    #[test]
    fn create_fixed() {
        let map: PrimitiveMap<u64, u8, _, _> = PrimitiveMap::with_fixed_bucket();
    }

    #[test]
    fn default_hasher() {
        let map: PrimitiveMap<u8, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<u16, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<u32, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<u64, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<usize, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<i8, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<i16, u8, _, _> = PrimitiveMap::with_fixed_bucket();
        let map: PrimitiveMap<i32, u8, _, _> = PrimitiveMap::with_fixed_bucket();
    }

    #[test]
    fn insert_dynamic() {
        let mut map: PrimitiveMap<_, _, _, _, DefaultHasher<_>> = PrimitiveMap::with_dynamic_bucket();
        map.insert(0u8, 10u32);
    }

    #[test]
    fn insert_fixed() {
        let mut map: PrimitiveMap<_, _, _, _, DefaultHasher<_>> = PrimitiveMap::with_fixed_bucket();
        map.insert(0u16, 10u32);
    }

    #[test]
    fn get_empty_dynamic() {
        let map: PrimitiveMap<_, u32, _, _, DefaultHasher<_>> = PrimitiveMap::with_dynamic_bucket();
        assert_eq!(map.get(0u32), None);
    }

    #[test]
    fn get_empty_fixed() {
        let map: PrimitiveMap<_, u32, _, _, DefaultHasher<_>>  = PrimitiveMap::with_fixed_bucket();
        assert_eq!(map.get(0u64), None);
    }

    #[test]
    fn insert_and_get_dynamic() {
        let mut map: PrimitiveMap<_, _, _, _, DefaultHasher<_>> = PrimitiveMap::with_dynamic_bucket();
        map.insert(0i8, 10u32);
        assert_eq!(map.get(0i8), Some(10u32));
    }

    #[test]
    fn insert_and_get_fixed() {
        let mut map: PrimitiveMap<_, _, _, _, DefaultHasher<_>> = PrimitiveMap::with_fixed_bucket();
        map.insert(0i16, 10u32);
        assert_eq!(map.get(0i16), Some(10u32));
    }

    #[test]
    fn insert_saturate_buckets_dynamic() {
        let mut map: PrimitiveMap<_, _, _, _, DefaultHasher<_>> = PrimitiveMap::with_dynamic_bucket();
        for i in 0..10000 {
            map.insert(i, 10u32);
        }
    }
}




