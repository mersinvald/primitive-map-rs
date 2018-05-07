use kv::{Key, Value};
use bucket::{DEFAULT_BUCKETS_COUNT, BucketStoreNew, Bucket};

impl<K, V, B> BucketStoreNew<K, V, B> for Vec<B>
    where
        K: Key,
        V: Value,
        B: Bucket<K, V> + 'static,
{
    fn initialized() -> Self {
        Self::initialized_with_capacity(DEFAULT_BUCKETS_COUNT)
    }

    fn initialized_with_capacity(cap: usize) -> Self {
        use std::iter;
        iter::repeat(B::new()).take(cap).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bucket::BucketStore;

    #[test]
    fn vec_bucket_list_with_flat_bucket() {
        let mut bl = vec![Some((1, 1)), Some((2, 2))];
        let bucket = bl.search_bucket(1, |bucket| bucket.map(|(k, _v)| k == 1).unwrap_or(false));
        assert_eq!(bucket.unwrap().unwrap(), (1, 1));
    }

    #[test]
    fn vec_bucket_list_with_vec_bucket() {
        let mut bl = vec![vec![(1, 1), (2, 2)], vec![(3, 3)]];
        let bucket = bl.search_bucket(1, |bucket| bucket.iter().any(|(k, _v)| *k == 2));
        assert_eq!(bucket.unwrap(), &vec![(1, 1), (2, 2)]);
    }
}
