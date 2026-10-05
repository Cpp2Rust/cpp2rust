extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0() -> i32 {
    let mut p: anon_1 = <anon_1>::default();
    p.x = 1;
    p.y = 2;
    return (p.x + p.y);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn second_2() -> i32 {
    let mut q: anon_3 = <anon_3>::default();
    q.a = 10_i64;
    q.b = 20_i64;
    return ((q.a + q.b) as i32);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct anon_3 {
    #[offset(0)]
    pub a: i64,
    #[offset(8)]
    pub b: i64,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ first_0() }) == 3) as i32) != 0));
    assert!((((({ second_2() }) == 30) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
