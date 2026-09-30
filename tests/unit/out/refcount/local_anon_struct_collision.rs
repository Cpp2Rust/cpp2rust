extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0() -> i32 {
    let p: Value<anon_1> = <Value<anon_1>>::default();
    (*p.borrow_mut()).x = 1;
    (*p.borrow_mut()).y = 2;
    return ({ (*p.borrow()).x } + { (*p.borrow()).y });
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn second_2() -> i32 {
    let q: Value<anon_3> = <Value<anon_3>>::default();
    (*q.borrow_mut()).a = 10_i64;
    (*q.borrow_mut()).b = 20_i64;
    return (({ (*q.borrow()).a } + { (*q.borrow()).b }) as i32);
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    #[offset(0)]
    pub a: i64,
    #[offset(8)]
    pub b: i64,
}
impl ByteRepr for anon_3 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..8]);
        self.b.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i64>::from_bytes(&buf[0..8]),
            b: <i64>::from_bytes(&buf[8..16]),
        }
    }
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
