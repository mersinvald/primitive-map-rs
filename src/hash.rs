use std::marker::PhantomData;
use std::{u8, u16};

pub trait Hash: Sized + Copy {}

impl Hash for u8 {}
impl Hash for i8 {}
impl Hash for u16 {}
impl Hash for i16 {}
impl Hash for u32 {}
impl Hash for i32 {}
impl Hash for u64 {}
impl Hash for usize {}

pub trait Hasher<T: Hash> {
    fn hash(value: T) -> T;
    fn compress(hash: T, upper_bound: usize) -> usize;
}

#[derive(Default)]
pub struct DefaultHasher<T: Copy> {
    _marker: PhantomData<T>
}

impl Hasher<u8> for DefaultHasher<u8> {
    fn hash(value: u8) -> u8 {
        value
    }

    fn compress(hash: u8, upper_bound: usize) -> usize {
        debug_assert!(upper_bound >= u8::MAX as usize);
        hash as usize
    }
}

impl Hasher<i8> for DefaultHasher<i8> {
    fn hash(value: i8) -> i8 {
        value
    }

    fn compress(hash: i8, upper_bound: usize) -> usize {
        debug_assert!(upper_bound >= u8::MAX as usize);
        hash as usize
    }
}

impl Hasher<u16> for DefaultHasher<u16> {
    fn hash(value: u16) -> u16{
        value ^ value >> 7
    }

    fn compress(hash: u16, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<i16> for DefaultHasher<i16> {
    fn hash(value: i16) -> i16 {
        value ^ value >> 7
    }

    fn compress(hash: i16, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<u32> for DefaultHasher<u32> {
    fn hash(value: u32) -> u32{
        value ^ value >> 7
    }

    fn compress(hash: u32, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<i32> for DefaultHasher<i32> {
    fn hash(value: i32) -> i32{
        value ^ value >> 7
    }

    fn compress(hash: i32, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<u64> for DefaultHasher<u64> {
    fn hash(value: u64) -> u64{
        value ^ value >> 7
    }

    fn compress(hash: u64, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}

impl Hasher<usize> for DefaultHasher<usize> {
    fn hash(value: usize) -> usize{
        value ^ value >> 7
    }

    fn compress(hash: usize, upper_bound: usize) -> usize {
        (hash as usize) % upper_bound
    }
}
