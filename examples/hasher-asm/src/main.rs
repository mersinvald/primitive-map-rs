#![feature(test)]
extern crate test;
extern crate primitivemap;

use primitivemap::Hasher;
use primitivemap::DefaultHasher;

#[inline(never)]
fn hash_u8(value: u8) -> usize {
    let value = DefaultHasher::hash(value);
    let value = DefaultHasher::compress(value, 256);
    value
}

#[inline(never)]
fn hash_u16(value: u16) -> usize {
    let value = DefaultHasher::hash(value);
    let value = DefaultHasher::compress(value, 1024);
    value
}

fn main() {
    println!("{}", hash_u8(test::black_box(100)));
    println!("{}", hash_u16(test::black_box(100)));
}
