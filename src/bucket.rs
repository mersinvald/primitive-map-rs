use kv::{Key, Value};
use smallvec::SmallVec;

pub const BUCKET_SIZE: usize = 2;
pub const BUCKET_LIST_SIZE: usize = 2048;

pub trait Bucket<K: Key, V: Value>: Sized {
    fn new() -> Self;
    fn push(&mut self, key: K, value: V);
    fn get(&self, key: K) -> Option<V>;
}

#[derive(Clone)]
pub struct SmallVecBucket<K: Key, V> {
    members: SmallVec<[(K, V); BUCKET_SIZE]>,
}

impl<K: Key, V: Value> Bucket<K, V> for SmallVecBucket<K, V> {
    fn new() -> Self {
        SmallVecBucket {
            members: SmallVec::new(),
        }
    }

    #[inline]
    fn push(&mut self, key: K, value: V) {
        self.members.push((key, value))
    }

    #[inline]
    fn get(&self, key: K) -> Option<V> {
        self.members
            .iter()
            .find(|&(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
}

#[derive(Copy, Clone)]
pub struct ArrayBucket<K: Key, V> {
    members: [(K, V); BUCKET_SIZE],
    len: usize,
}

impl<K: Key, V: Value> Bucket<K, V> for ArrayBucket<K, V> {
    fn new() -> Self {
        ArrayBucket {
            members: [(K::default(), V::default()); BUCKET_SIZE],
            len: 0,
        }
    }

    #[inline]
    fn push(&mut self, key: K, value: V) {
        let idx = self.len;
        self.members[idx] = (key, value);
        self.len += 1;
    }

    #[inline]
    fn get(&self, key: K) -> Option<V> {
        let len = self.len;
        self.members[..len]
            .iter()
            .find(|&(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
}

pub type SmallVecBucketList<K, V> = SmallVec<[SmallVecBucket<K, V>; BUCKET_LIST_SIZE]>;
pub type ArrayBucketList<K, V> = [ArrayBucket<K, V>; BUCKET_LIST_SIZE];

pub trait BucketList<K: Key, V: Value> {
    type Bucket: Bucket<K, V>;

    fn empty() -> Self;
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> &Self::Bucket;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket;
}

impl<K: Key, V: Value> BucketList<K, V> for SmallVecBucketList<K, V> {
    type Bucket = SmallVecBucket<K, V>;

    fn empty() -> Self {
        let mut vec = SmallVec::new();
        // pre-polulate vec
        for _ in 0..BUCKET_LIST_SIZE {
            vec.push(SmallVecBucket::new())
        }
        vec
    }

    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    #[inline]
    fn get(&self, idx: usize) -> &Self::Bucket {
        &self[idx]
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket {
        &mut self[idx]
    }
}

impl<K: Key, V: Value> BucketList<K, V> for ArrayBucketList<K, V> {
    type Bucket = ArrayBucket<K, V>;

    fn empty() -> Self {
        [ArrayBucket::new(); BUCKET_LIST_SIZE]
    }

    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    #[inline]
    fn get(&self, idx: usize) -> &Self::Bucket {
        &self[idx]
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut Self::Bucket {
        &mut self[idx]
    }
}
