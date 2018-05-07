pub mod vec;
pub mod array;

pub use self::array::*;

use bucket::{Bucket, BucketStore};
use kv::{Key, Value};
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::usize;
use bucket::helpers::WrappingIndexIterator;


impl<K: Key, V: Value, B: Bucket<K, V>> BucketStore<K, V, B> for [B] {
    #[inline]
    fn len(&self) -> usize {
        <[B]>::len(self)
    }

    #[inline]
    fn get(&self, idx: usize) -> &B {
        &self[idx]
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut B {
        &mut self[idx]
    }

    fn search<P: Fn(&B) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&B> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            let bucket = unsafe { self.get_unchecked(i) };

            if predicate(bucket) {
                return Some(bucket);
            }
        }

        None
    }

    fn search_mut<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B> {
        for i in WrappingIndexIterator::new(start_idx, self.len()) {
            // It is safe here as we do not have indexes outside of [0; len)
            // T_T
            // borrowck was mad about the mutable version of search,
            // so I was forced to rewrite it this awful way
            if predicate(unsafe { self.get_unchecked_mut(i) }) {
                return Some(unsafe { self.get_unchecked_mut(i) });
            }
        }

        None
    }
}


impl<T, K: Key, V: Value, B: Bucket<K, V>> BucketStore<K, V, B> for T
    where
        T: AsRef<[B]> + AsMut<[B]>,
{
    #[inline]
    fn len(&self) -> usize {
        BucketStore::<K, V, B>::len(self.as_ref())
    }

    #[inline]
    fn get(&self, idx: usize) -> &B {
        BucketStore::<K, V, B>::get(self.as_ref(), idx)
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut B {
        BucketStore::<K, V, B>::get_mut(self.as_mut(), idx)
    }

    #[inline]
    fn search<P: Fn(&B) -> bool>(&self, start_idx: usize, predicate: P) -> Option<&B> {
        self.as_ref().search(start_idx, predicate)
    }

    #[inline]
    fn search_mut<P: Fn(&B) -> bool>(&mut self, start_idx: usize, predicate: P) -> Option<&mut B> {
        self.as_mut().search_mut(start_idx, predicate)
    }
}
