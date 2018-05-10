#![feature(step_trait)]
#[macro_use]
extern crate criterion;
extern crate indexmap;
extern crate primitivemap;

use criterion::black_box;
use criterion::Criterion;

use criterion::Fun;
use primitivemap::bucket::{Array1024, Array256};
use primitivemap::kv::Key;
use primitivemap::Hasher;
use primitivemap::{ArrayPrimitiveMap, LinearPrimitiveMap, PrimitiveMap, VecPrimitiveMap};
use primitivemap::{Bucket, BucketStore, BucketStoreNew};
use std::{u16, u64, u8};

const U8_CAP: usize = u8::max_value() as usize;
const U8_MAX: u8 = u8::max_value();
const U16_CAP: usize = u16::max_value() as usize;
const U16_MAX: u16 = u16::max_value();

const LOW_LOAD_CAP: usize = 1024;
const LOW_LOAD_MAX: usize = 256;
const OVERLOAD_CAP: usize = 256;
const OVERLOAD_MAX: usize = 8192;

use std::fmt::Debug;
use std::iter::Step;

fn pmap_fun_insert<
    K: Key + Copy + Step + Debug + 'static,
    B: Bucket<K, K> + 'static,
    BL: BucketStore<K, K, B> + Clone + 'static,
    H: Hasher<K> + 'static,
>(
    name: &'static str,
    map: PrimitiveMap<K, K, B, BL, H>,
) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| {
        b.iter_with_setup(
            || map.clone(),
            |mut map| {
                for i in low..high {
                    map.insert(i, i);
                }
            },
        )
    })
}

fn pmap_fun_get<
    K: Key + Copy + Step + Debug + 'static,
    B: Bucket<K, K> + 'static,
    BL: BucketStore<K, K, B> + Clone + 'static,
    H: Hasher<K> + 'static,
>(
    name: &'static str,
    map: PrimitiveMap<K, K, B, BL, H>,
) -> Fun<(K, K)> {
    Fun::new(name, move |b, &(low, high)| {
        b.iter_with_setup(
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
            },
        )
    })
}

fn bench_u8(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(U8_MAX);
    c.bench_functions(
        &format!("Key: u8, Capacity: {}, Load: {}", U8_CAP, U8_MAX),
        vec![
            pmap_fun_insert(
                "VecPrimitiveMap [INSERT]",
                VecPrimitiveMap::with_capacity(U8_CAP),
            ),
            pmap_fun_insert(
                "ArrayPrimitiveMap [INSERT]",
                ArrayPrimitiveMap::with_buckets(Array256::initialized()),
            ),
            pmap_fun_insert(
                "LinearPrimitiveMap [INSERT]",
                LinearPrimitiveMap::with_buckets(Array256::initialized()),
            ),
            pmap_fun_get(
                "VecPrimitiveMap [GET]",
                VecPrimitiveMap::with_capacity(U8_CAP),
            ),
            pmap_fun_get(
                "ArrayPrimitiveMap [GET]",
                ArrayPrimitiveMap::with_buckets(Array256::initialized()),
            ),
            pmap_fun_get(
                "LinearPrimitiveMap [GET]",
                LinearPrimitiveMap::with_buckets(Array256::initialized()),
            ),
        ],
        (low, high),
    );
}

fn bench_u16(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(U16_MAX);
    c.bench_functions(
        &format!("Key: u16, Capacity: {}, Load: {}", U16_CAP, U16_MAX),
        vec![
            pmap_fun_insert(
                "VecPrimitiveMap [INSERT]",
                VecPrimitiveMap::with_capacity(U16_CAP),
            ),
            pmap_fun_get(
                "VecPrimitiveMap [GET]",
                VecPrimitiveMap::with_capacity(U16_CAP),
            ),
        ],
        (low, high),
    );
}

fn bench_u64_low_load(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(LOW_LOAD_MAX as u64);
    c.bench_functions(
        &format!(
            "Key: u64, Capacity: {}, Load: {}",
            LOW_LOAD_CAP, LOW_LOAD_MAX
        ),
        vec![
            pmap_fun_insert(
                "VecPrimitiveMap [INSERT]",
                VecPrimitiveMap::with_capacity(LOW_LOAD_CAP),
            ),
            pmap_fun_insert(
                "ArrayPrimitiveMap [INSERT]",
                ArrayPrimitiveMap::with_buckets(Array1024::initialized()),
            ),
            pmap_fun_insert(
                "LinearPrimitiveMap [INSERT]",
                LinearPrimitiveMap::with_buckets(Array1024::initialized()),
            ),
            pmap_fun_get(
                "VecPrimitiveMap [GET]",
                VecPrimitiveMap::with_capacity(LOW_LOAD_CAP),
            ),
            pmap_fun_get(
                "ArrayPrimitiveMap [GET]",
                ArrayPrimitiveMap::with_buckets(Array1024::initialized()),
            ),
            pmap_fun_get(
                "LinearPrimitiveMap [GET]",
                LinearPrimitiveMap::with_buckets(Array1024::initialized()),
            ),
        ],
        (low, high),
    );
}

fn bench_u64_overload(c: &mut Criterion) {
    let low = black_box(0);
    let high = black_box(OVERLOAD_MAX as u64);
    c.bench_functions(
        &format!(
            "Key: u64, Capacity: {}, Load: {}",
            OVERLOAD_CAP, OVERLOAD_MAX
        ),
        vec![
            pmap_fun_insert(
                "VecPrimitiveMap [INSERT]",
                VecPrimitiveMap::with_capacity(OVERLOAD_CAP),
            ),
            pmap_fun_insert(
                "ArrayPrimitiveMap [INSERT]",
                ArrayPrimitiveMap::with_buckets(Array256::initialized()),
            ),
            pmap_fun_get(
                "VecPrimitiveMap [GET]",
                VecPrimitiveMap::with_capacity(OVERLOAD_CAP),
            ),
            pmap_fun_get(
                "ArrayPrimitiveMap [GET]",
                ArrayPrimitiveMap::with_buckets(Array256::initialized()),
            ),
        ],
        (low, high),
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
