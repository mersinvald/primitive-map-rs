#![feature(test, step_trait)]
extern crate indexmap;
extern crate test;

extern crate primitivemap;

#[cfg(test)]
mod tests {
    use primitivemap::{PrimitiveMap, VecPrimitiveMap, ArrayPrimitiveMap, LinearPrimitiveMap};
    use primitivemap::bucket::{Array256, Array1024};
    use primitivemap::kv::Key;
    use primitivemap::Hasher;
    use primitivemap::{Bucket, BucketList, BucketListNew};

    use indexmap::IndexMap;
    use std::{collections::HashMap, u16, u32, u64, u8};
    use test;

    const LOW_LOAD_BATCH_SIZE: usize = 1024;

    use std::iter::Step;
    use std::fmt::Debug;
    fn bench_generic_pmap<T: Key + Step + Debug, B: Bucket<T, T>, BL: BucketList<T, T, B> + Clone, H: Hasher<T>>(low:T, high: T, map: PrimitiveMap<T, T, B, BL, H>, b: &mut test::Bencher) {
        let low = test::black_box(low);
        let high = test::black_box(high);
        b.iter(|| {
            let mut map = map.clone();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u8_vec_primitive_map(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u8, u8::MAX,
            VecPrimitiveMap::with_capacity(u8::MAX as usize),
            b
        );
    }

    #[bench]
    fn bench_u8_array_primitive_map(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u8, u8::MAX,
            ArrayPrimitiveMap::with_buckets(Array256::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u8_linear_primitive_map(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u8, u8::MAX,
            LinearPrimitiveMap::with_buckets(Array256::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u8_indexmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u8);
        let high = test::black_box(u8::MAX);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u8_std_hashmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u8);
        let high = test::black_box(u8::MAX);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u16_vec_primitive_map(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u16, u16::MAX,
            VecPrimitiveMap::with_capacity(u16::MAX as usize),
            b
        );
    }

    // Note: this test saturates buckets __real hard__, effectively turning algorithm into O(n)
    #[bench]
    fn bench_u16_array_primitive_map(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u16, u16::MAX,
            ArrayPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u16_indexmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(u16::MAX);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u16_std_hashmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(u16::MAX);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u16_vec_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u16, LOW_LOAD_BATCH_SIZE as u16,
            VecPrimitiveMap::with_capacity(LOW_LOAD_BATCH_SIZE),
            b
        );
    }

    #[bench]
    fn bench_u16_array_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u16, LOW_LOAD_BATCH_SIZE as u16,
            ArrayPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u16_linear_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u16, LOW_LOAD_BATCH_SIZE as u16,
            LinearPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u16_std_hashmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(LOW_LOAD_BATCH_SIZE as u16);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u16_indexmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(LOW_LOAD_BATCH_SIZE as u16);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u32_vec_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u32, LOW_LOAD_BATCH_SIZE as u32,
            VecPrimitiveMap::with_capacity(LOW_LOAD_BATCH_SIZE),
            b
        );
    }

    #[bench]
    fn bench_u32_array_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u32, LOW_LOAD_BATCH_SIZE as u32,
            ArrayPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u32_linear_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u32, LOW_LOAD_BATCH_SIZE as u32,
            LinearPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u32_std_hashmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(LOW_LOAD_BATCH_SIZE as u32);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u32_indexmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(LOW_LOAD_BATCH_SIZE as u32);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u64_vec_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u64, LOW_LOAD_BATCH_SIZE as u64,
            VecPrimitiveMap::with_capacity(LOW_LOAD_BATCH_SIZE),
            b
        );
    }

    #[bench]
    fn bench_u64_array_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u64, LOW_LOAD_BATCH_SIZE as u64,
            ArrayPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u64_linear_primitive_map_low_load(b: &mut test::Bencher) {
        bench_generic_pmap(
            0_u64, LOW_LOAD_BATCH_SIZE as u64,
            LinearPrimitiveMap::with_buckets(Array1024::initialized()),
            b
        );
    }

    #[bench]
    fn bench_u64_std_hashmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(LOW_LOAD_BATCH_SIZE as u64);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }

    #[bench]
    fn bench_u64_indexmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(LOW_LOAD_BATCH_SIZE as u64);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, i);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&i));
            }
        })
    }
}
