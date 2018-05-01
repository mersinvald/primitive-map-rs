#![feature(test)]
extern crate indexmap;
extern crate test;

extern crate primitivemap;

#[cfg(test)]
mod tests {
    use primitivemap::{
        bucket::BUCKET_LIST_SIZE,
        PrimitiveMap
    };

    use indexmap::IndexMap;
    use std::{collections::HashMap, u16, u32, u64, u8};
    use test;

    #[bench]
    fn bench_u8_dynamic(b: &mut test::Bencher) {
        let low = test::black_box(0_u8);
        let high = test::black_box(u8::MAX);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u8_fixed(b: &mut test::Bencher) {
        let low = test::black_box(0_u8);
        let high = test::black_box(u8::MAX);
        b.iter(|| {
            let mut map =
                PrimitiveMap::fixed();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u8_indexmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u8);
        let high = test::black_box(u8::MAX);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
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
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u16_dynamic(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(u16::MAX);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u16_indexmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(u16::MAX);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
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
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u16_dynamic_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(BUCKET_LIST_SIZE as u16);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u16_fixed_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(BUCKET_LIST_SIZE as u16);
        b.iter(|| {
            let mut map =
                PrimitiveMap::fixed();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u16_std_hashmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(BUCKET_LIST_SIZE as u16);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u16_indexmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u16);
        let high = test::black_box(BUCKET_LIST_SIZE as u16);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[ignore]
    #[bench]
    fn bench_u32_dynamic(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(u32::MAX);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[ignore]
    #[bench]
    fn bench_u32_std_hashmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(u32::MAX);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u32_dynamic_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(BUCKET_LIST_SIZE as u32);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u32_fixed_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(BUCKET_LIST_SIZE as u32);
        b.iter(|| {
            let mut map =
                PrimitiveMap::fixed();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u32_std_hashmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(BUCKET_LIST_SIZE as u32);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u32_indexmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u32);
        let high = test::black_box(BUCKET_LIST_SIZE as u32);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[ignore]
    #[bench]
    fn bench_u64_dynamic(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(u64::MAX);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[ignore]
    #[bench]
    fn bench_u64_std_hashmap(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(u64::MAX);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u64_dynamic_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(BUCKET_LIST_SIZE as u64);
        b.iter(|| {
            let mut map =
                PrimitiveMap::dynamic();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u64_fixed_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(BUCKET_LIST_SIZE as u64);
        b.iter(|| {
            let mut map =
                PrimitiveMap::fixed();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(i), Some(0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u64_std_hashmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(BUCKET_LIST_SIZE as u64);
        b.iter(|| {
            let mut map = HashMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }

    #[bench]
    fn bench_u64_indexmap_low_load(b: &mut test::Bencher) {
        let low = test::black_box(0_u64);
        let high = test::black_box(BUCKET_LIST_SIZE as u64);
        b.iter(|| {
            let mut map = IndexMap::new();
            for i in low..high {
                map.insert(i, 0xFFFF);
            }
            for i in low..high {
                assert_eq!(map.get(&i), Some(&0xFFFF));
            }
        })
    }
}
