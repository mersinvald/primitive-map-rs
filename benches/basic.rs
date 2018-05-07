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
use std::{u8, u16, u32, u64};
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
    Fun::new(name, move |b, &(low, high)| b.iter(|| {
        let mut map = map.clone();
        for i in low..high {
            map.insert(i, i);
        }
    }))
}

fn bench_u8(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(u8::MAX);
    c.bench_functions(
        &format!("[INSERT] Key: u8, Capacity: {}, Load: {}", u8::MAX, u8::MAX),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(u8::MAX as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
            pmap_fun_insert("LinearPrimitiveMap", LinearPrimitiveMap::with_buckets(Array256::initialized())),
        ],
        (low, high)
    );
}

fn bench_u16(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(u16::MAX);
    c.bench_functions(
        &format!("[INSERT] Key: u16, Capacity: {}, Load: {}", u16::MAX, u16::MAX),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(u16::MAX as usize)),
        ],
        (low, high)
    );
}

fn bench_u16_low_load(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(LOW_LOAD_BATCH_SIZE as u16);
    c.bench_functions(
        &format!("[INSERT] Key: u16, Capacity: {}, Load: {}", LOW_LOAD_MAP_SIZE, LOW_LOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array1024::initialized())),
            pmap_fun_insert("LinearPrimitiveMap", LinearPrimitiveMap::with_buckets(Array1024::initialized())),
        ],
        (low, high)
    );
}

fn bench_u16_overload(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(OVERLOAD_BATCH_SIZE as u16);
    c.bench_functions(
        &format!("[INSERT] Key: u16, Capacity: {}, Load: {}", OVERLOAD_MAP_SIZE, OVERLOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
        ],
        (low, high)
    );
}

fn bench_u32_low_load(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(LOW_LOAD_BATCH_SIZE as u32);
    c.bench_functions(
        &format!("[INSERT] Key: u32, Capacity: {}, Load: {}", LOW_LOAD_MAP_SIZE, LOW_LOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array1024::initialized())),
            pmap_fun_insert("LinearPrimitiveMap", LinearPrimitiveMap::with_buckets(Array1024::initialized())),
        ],
        (low, high)
    );
}

fn bench_u32_overload(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(OVERLOAD_BATCH_SIZE as u32);
    c.bench_functions(
        &format!("[INSERT] Key: u32, Capacity: {}, Load: {}", OVERLOAD_MAP_SIZE, OVERLOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
        ],
        (low, high)
    );
}

fn bench_u64_low_load(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(LOW_LOAD_BATCH_SIZE as u64);
    c.bench_functions(
        &format!("[INSERT] Key: u64, Capacity: {}, Load: {}", LOW_LOAD_MAP_SIZE, LOW_LOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(LOW_LOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array1024::initialized())),
            pmap_fun_insert("LinearPrimitiveMap", LinearPrimitiveMap::with_buckets(Array1024::initialized())),
        ],
        (low, high)
    );
}

fn bench_u64_overload(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(OVERLOAD_BATCH_SIZE as u64);
    c.bench_functions(
        &format!("[INSERT] Key: u64, Capacity: {}, Load: {}", OVERLOAD_MAP_SIZE, OVERLOAD_BATCH_SIZE),
        vec![
            pmap_fun_insert("VecPrimitiveMap", VecPrimitiveMap::with_capacity(OVERLOAD_MAP_SIZE as usize)),
            pmap_fun_insert("ArrayPrimitiveMap", ArrayPrimitiveMap::with_buckets(Array256::initialized())),
        ],
        (low, high)
    );
}

use std::time::Duration;

fn criterion_config() -> Criterion {
    Criterion::default()
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
    targets = bench_u16_low_load, bench_u32_low_load, bench_u64_low_load
}

criterion_group! {
    name = benches_overload;
    config = criterion_config();
    targets = bench_u16_overload, bench_u32_overload, bench_u64_overload
}

criterion_main!(benches_u8, benches_u16, benches_low_load, benches_overload);
