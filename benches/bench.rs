#![feature(step_trait)]
#[macro_use]
extern crate criterion;
extern crate indexmap;
extern crate primitivemap;

use criterion::Criterion;
use criterion::black_box;

use primitivemap::{PrimitiveMap, VecPrimitiveMap, ArrayPrimitiveMap, LinearPrimitiveMap};
use primitivemap::bucket::{Array256, Array1024};
use primitivemap::kv::Key;
use primitivemap::Hasher;
use primitivemap::{Bucket, BucketStore, BucketStoreNew};
use std::{u8, u16, u64};
use criterion::Fun;

use indexmap::IndexMap;
use std::{collections::HashMap};

const LOW_LOAD_MAP_SIZE: usize = 1024;
const LOW_LOAD_BATCH_SIZE: usize = 256;
const OVERLOAD_MAP_SIZE: usize = 256;
const OVERLOAD_BATCH_SIZE: usize = 8192;

use std::iter::Step;
use std::fmt::Debug;


fn pmap_fun_insert<K: Key + Copy + Step + Debug + 'static, B: Bucket<K, K> + 'static, BL: BucketStore<K, K, B> + Clone + 'static, H: Hasher<K> + 'static>(name: &'static str, map: PrimitiveMap<K, K, B, BL, H>) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| b.iter_with_setup(
        || map.clone(),
        |mut map| {
            for i in low..high {
                map.insert(i, i);
        }
    }))
}

fn pmap_fun_get<K: Key + Copy + Step + Debug + 'static, B: Bucket<K, K> + 'static, BL: BucketStore<K, K, B> + Clone + 'static, H: Hasher<K> + 'static>(name: &'static str, map: PrimitiveMap<K, K, B, BL, H>) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| b.iter_with_setup(
        || {
            let mut map = map.clone();
            for i in low..high {
                map.insert(i, i);
            }
            map
        },
        |map| {
            for i in low..high {
                assert_eq!(map.get(i), Some(&i));
            }
    }))
}

fn indexmap_fun_insert<K: std::hash::Hash + Eq + Copy + Step + Debug + 'static>(name: &'static str, map: IndexMap<K, K>) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| b.iter_with_setup(
    || map.clone(),
    |mut map| {
        for i in low..high {
            map.insert(i, i);
        }
    }))
}

fn indexmap_fun_get<K: std::hash::Hash + Eq + Copy + Step + Debug + 'static>(name: &'static str, map: IndexMap<K, K>) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| b.iter_with_setup(
    || {
        let mut map = map.clone();
        for i in low..high {
            map.insert(i, i);
        }
        map
    },
    |map| {
        for i in low..high {
            assert_eq!(map.get(&i), Some(&i));
        }
    }))
}

fn std_hashmap_fun_insert<K: std::hash::Hash + Eq + Copy + Step + Debug + 'static>(name: &'static str, map: HashMap<K, K>) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| b.iter_with_setup(
    || map.clone(),
    |mut map| {
        for i in low..high {
            map.insert(i, i);
        }
    }))
}

fn std_hashmap_fun_get<K: std::hash::Hash + Eq + Copy + Step + Debug + 'static>(name: &'static str, map: HashMap<K, K>) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| b.iter_with_setup(
    || {
        let mut map = map.clone();
        for i in low..high {
            map.insert(i, i);
        }
        map
    },
    |map| {
        for i in low..high {
            assert_eq!(map.get(&i), Some(&i));
        }
    }))
}


fn bench_u8(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(u8::MAX);
    c.bench_functions(
        &format!("Key: u8, Capacity: {}, Load: {}", u8::MAX, u8::MAX),
        vec![
            pmap_fun_insert("VecPrimitiveMap [INSERT]", VecPrimitiveMap::with_capacity(u8::MAX as usize)),
            pmap_fun_insert("ArrayPrimitiveMap [INSERT]", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
            pmap_fun_insert("LinearPrimitiveMap [INSERT]", LinearPrimitiveMap::with_buckets(Array256::initialized())),
            indexmap_fun_insert("IndexMap [INSERT]", IndexMap::with_capacity(u8::MAX as usize)),
            std_hashmap_fun_insert("StdHashMap [INSERT]", HashMap::with_capacity(u8::MAX as usize)),

            pmap_fun_get("VecPrimitiveMap [GET]", VecPrimitiveMap::with_capacity(u8::MAX as usize)),
            pmap_fun_get("ArrayPrimitiveMap [GET]", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
            pmap_fun_get("LinearPrimitiveMap [GET]", LinearPrimitiveMap::with_buckets(Array256::initialized())),
            indexmap_fun_get("IndexMap [GET]", IndexMap::with_capacity(u8::MAX as usize)),
            std_hashmap_fun_get("StdHashMap [GET]", HashMap::with_capacity(u8::MAX as usize)),
        ],
        (low, high)
    );
}

fn bench_u16(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(u16::MAX);
    c.bench_functions(
        &format!("Key: u16, Capacity: {}, Load: {}", u16::MAX, u16::MAX),
        vec![
            pmap_fun_insert("VecPrimitiveMap [INSERT]", VecPrimitiveMap::with_capacity(u16::MAX as usize)),
            indexmap_fun_insert("IndexMap [INSERT]", IndexMap::with_capacity(u16::MAX as usize)),
            std_hashmap_fun_insert("StdHashMap [INSERT]", HashMap::with_capacity(u16::MAX as usize)),

            pmap_fun_get("VecPrimitiveMap [GET]", VecPrimitiveMap::with_capacity(u16::MAX as usize)),
            indexmap_fun_get("IndexMap [GET]", IndexMap::with_capacity(u16::MAX as usize)),
            std_hashmap_fun_get("StdHashMap [GET]", HashMap::with_capacity(u16::MAX as usize)),
        ],
        (low, high)
    );
}

fn bench_u64_low_load(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(LOW_LOAD_BATCH_SIZE as u64);
    c.bench_functions(
        &format!("Key: u64, Capacity: {}, Load: {}", LOW_LOAD_MAP_SIZE, LOW_LOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap [INSERT]", VecPrimitiveMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap [INSERT]", ArrayPrimitiveMap::with_buckets(Array1024::initialized())),
            pmap_fun_insert("LinearPrimitiveMap [INSERT]", LinearPrimitiveMap::with_buckets(Array1024::initialized())),
            indexmap_fun_insert("IndexMap [INSERT]", IndexMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            std_hashmap_fun_insert("StdHashMap [INSERT]", HashMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),

            pmap_fun_get("VecPrimitiveMap [GET]", VecPrimitiveMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            pmap_fun_get("ArrayPrimitiveMap [GET]", ArrayPrimitiveMap::with_buckets(Array1024::initialized())),
            pmap_fun_get("LinearPrimitiveMap [GET]", LinearPrimitiveMap::with_buckets(Array1024::initialized())),
            indexmap_fun_get("IndexMap [GET]", IndexMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            std_hashmap_fun_get("StdHashMap [GET]", HashMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
        ],
        (low, high)
    );
}

fn bench_u64_overload(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(OVERLOAD_BATCH_SIZE as u64);
    c.bench_functions(
        &format!("Key: u64, Capacity: {}, Load: {}", OVERLOAD_MAP_SIZE, OVERLOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap [INSERT]", VecPrimitiveMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap [INSERT]", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
            indexmap_fun_insert("IndexMap [INSERT]", IndexMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            std_hashmap_fun_insert("StdHashMap [INSERT]", HashMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),

            pmap_fun_get("VecPrimitiveMap [GET]", VecPrimitiveMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            pmap_fun_get("ArrayPrimitiveMap [GET]", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
            indexmap_fun_get("IndexMap [GET]", IndexMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            std_hashmap_fun_get("StdHashMap [GET]", HashMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
        ],
        (low, high)
    );
}

use std::time::Duration;

fn criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(400)
}

criterion_group! {
    name = benches_u8;
    config = criterion_config();
    targets = bench_u8
}

criterion_group!(benches_u16, bench_u16);

criterion_group! {
    name = benches_low_load;
    config = criterion_config();
    targets = bench_u64_low_load
}

criterion_group! {
    name = benches_overload;
    config = criterion_config();
    targets = bench_u64_overload
}

criterion_main!(benches_u8, benches_u16, benches_low_load, benches_overload);
