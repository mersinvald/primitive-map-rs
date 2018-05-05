use std::marker::PhantomData;
use std::{u16, u8};

pub trait Hash: Sized + Copy {}

impl Hash for u8 {}
impl Hash for i8 {}
impl Hash for u16 {}
impl Hash for i16 {}
impl Hash for u32 {}
impl Hash for i32 {}
impl Hash for u64 {}
impl Hash for usize {}

pub trait Hasher<T: Hash>: Default {
    fn hash(value: T) -> T;
    fn compress(hash: T, upper_bound: usize) -> usize;
}

#[derive(Default)]
pub struct DefaultHasher<T: Hash> {
    _marker: PhantomData<T>,
}

impl<T: Hash> DefaultHasher<T> {
    pub fn new() -> Self {
        DefaultHasher {
            _marker: PhantomData,
        }
    }
}

impl Hasher<u8> for DefaultHasher<u8> {
    #[inline]
    fn hash(value: u8) -> u8 {
        value
    }

    #[inline]
    fn compress(hash: u8, upper_bound: usize) -> usize {
        hash as usize % upper_bound
    }
}

impl Hasher<i8> for DefaultHasher<i8> {
    #[inline]
    fn hash(value: i8) -> i8 {
        value
    }

    #[inline]
    fn compress(hash: i8, upper_bound: usize) -> usize {
        hash as usize % upper_bound
    }
}

impl Hasher<u16> for DefaultHasher<u16> {
    #[inline]
    fn hash(value: u16) -> u16 {
        value ^ value >> 7
    }

    #[inline]
    fn compress(hash: u16, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<i16> for DefaultHasher<i16> {
    #[inline]
    fn hash(value: i16) -> i16 {
        value ^ value >> 7
    }

    #[inline]
    fn compress(hash: i16, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<u32> for DefaultHasher<u32> {
    #[inline]
    fn hash(value: u32) -> u32 {
        value ^ value >> 7
    }

    #[inline]
    fn compress(hash: u32, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<i32> for DefaultHasher<i32> {
    #[inline]
    fn hash(value: i32) -> i32 {
        value ^ value >> 7
    }

    #[inline]
    fn compress(hash: i32, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<u64> for DefaultHasher<u64> {
    #[inline]
    fn hash(value: u64) -> u64 {
        value ^ value >> 7
    }

    #[inline]
    fn compress(hash: u64, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<usize> for DefaultHasher<usize> {
    #[inline]
    fn hash(value: usize) -> usize {
        value ^ value >> 7
    }

    #[inline]
    fn compress(hash: usize, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_hasher_type_support() {
        DefaultHasher::hash(1_u8);
        DefaultHasher::hash(1_u16);
        DefaultHasher::hash(1_u32);
        DefaultHasher::hash(1_u64);
        DefaultHasher::hash(1_usize);
        DefaultHasher::hash(1_i8);
        DefaultHasher::hash(1_i16);
        DefaultHasher::hash(1_i32);
    }
}
