extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_2() -> i32 {
    let p: Value<anon_0> = <Value<anon_0>>::default();
    (*p.borrow_mut()).x = 1;
    (*p.borrow_mut()).y = 2;
    return ({ (*p.borrow()).x } + { (*p.borrow()).y });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn second_3() -> i32 {
    let q: Value<anon_1> = <Value<anon_1>>::default();
    (*q.borrow_mut()).a = 10_i64;
    (*q.borrow_mut()).b = 20_i64;
    return (({ (*q.borrow()).a } + { (*q.borrow()).b }) as i32);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct anon_1 {
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
    assert!((((({ first_2() }) == 3) as i32) != 0));
    assert!((((({ second_3() }) == 30) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
