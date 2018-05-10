use bucket::{Bucket, BucketStore, BucketStoreNew, DEFAULT_BUCKETS_COUNT};
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::mem;
use std::usize;

macro_rules! impl_bucket_list_for_array {
    ($name:ident, $size:expr) => {
        #[derive(Clone)]
        pub struct $name<K: Key, V: Value, B: Bucket<K, V>> {
            inner: [B; $size],
            _marker: PhantomData<(K, V)>,
        }

        impl<K: Key, V: Value, B: Bucket<K, V> + 'static> BucketStoreNew<K, V, B> for $name<K, V, B> {
            fn initialized() -> Self {
                unsafe {
                    let mut array: [B; $size] = mem::uninitialized();

                    let aptr = array[..].as_mut_ptr();
                    for i in 0..$size {
                        aptr.offset(i).write(B::new())
                    }

                    $name {
                        inner: array,
                        _marker: PhantomData,
                    }
                }
            }

            fn initialized_with_capacity(cap: usize) -> Self {
                panic!("invalid capacity({}): {} is static on-stack structure, thus it has no capacity configureble in runtime. `initialized` should be used instead.", cap, stringify!($name));
            }
        }

        impl<K, V, B> AsRef<[B]> for $name<K, V, B>
        where
            K: Key,
            V: Value,
            B: Bucket<K, V>,
        {
            #[inline]
            fn as_ref(&self) -> &[B] {
                &self.inner[..]
            }
        }

        impl<K, V, B> AsMut<[B]> for $name<K, V, B>
        where
            K: Key,
            V: Value,
            B: Bucket<K, V>,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [B] {
                &mut self.inner[..]
            }
        }
    };
}

impl_bucket_list_for_array!(Array2, 2);
impl_bucket_list_for_array!(Array4, 4);
impl_bucket_list_for_array!(Array8, 8);
impl_bucket_list_for_array!(Array16, 16);
impl_bucket_list_for_array!(Array32, 32);
impl_bucket_list_for_array!(Array64, 64);
impl_bucket_list_for_array!(Array128, 128);
impl_bucket_list_for_array!(Array256, 256);
impl_bucket_list_for_array!(Array512, 512);
impl_bucket_list_for_array!(Array768, 768);
impl_bucket_list_for_array!(Array1024, 1024);
impl_bucket_list_for_array!(Array2048, 2048);
impl_bucket_list_for_array!(Array4096, 4096);
impl_bucket_list_for_array!(Array8192, 8192);
impl_bucket_list_for_array!(Array16384, 16384);
impl_bucket_list_for_array!(Array32768, 32768);
