extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Default for Inner {
    fn default() -> Self {
        Inner { x: 3, y: 4 }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i8,
    #[offset(8)]
    #[byte_size(8)]
    pub c: Inner,
    #[offset(16)]
    #[byte_size(8)]
    pub d: Inner,
}
impl Default for S {
    fn default() -> Self {
        S {
            a: 1,
            b: 2_i8,
            c: <Inner>::default(),
            d: <Inner>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Boxed_int_ {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub tag: i32,
}
impl Boxed_int_ {
    pub fn new(mut x: i32, mut t: i32) -> Self {
        Self { v: x, tag: t }
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        Boxed_int_ {
            v: 0_i32,
            tag: 0_i32,
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut s: S = <S>::default();
    assert!((s.a == 1));
    assert!(((s.b as i32) == 2));
    assert!((s.c.x == 3));
    assert!((s.c.y == 4));
    assert!((s.d.x == 3));
    assert!((s.d.y == 4));
    let mut boxed: Boxed_int_ = Boxed_int_::new({ 5 }, { 9 });
    assert!((boxed.v == 5));
    assert!((boxed.tag == 9));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
