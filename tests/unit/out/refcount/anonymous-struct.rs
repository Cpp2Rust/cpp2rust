extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_Named {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub c: i32,
    #[offset(4)]
    pub d: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub g: i32,
    #[offset(4)]
    pub h: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_2 {
    #[offset(0)]
    pub e: i32,
    #[offset(4)]
    pub f: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_3 {
    #[offset(0)]
    pub j: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_4 {
    #[offset(0)]
    pub k: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct anon_5 {
    #[offset(0)]
    pub i: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner_named: anon_3,
    #[offset(8)]
    #[byte_size(4)]
    pub anon_4: anon_4,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(44)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub named: Outer_Named,
    #[offset(8)]
    #[byte_size(8)]
    pub anonymous_named_0: anon_0,
    #[offset(16)]
    #[byte_size(8)]
    pub anonymous_named_1: anon_1,
    #[offset(24)]
    #[byte_size(8)]
    pub anon_2: anon_2,
    #[offset(32)]
    #[byte_size(12)]
    pub anon_5: anon_5,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    (*o.borrow_mut()).named.a = 1;
    (*o.borrow_mut()).named.b = 2;
    (*o.borrow_mut()).anonymous_named_0.c = 3;
    (*o.borrow_mut()).anonymous_named_0.d = 4;
    (*o.borrow_mut()).anonymous_named_1.g = 5;
    (*o.borrow_mut()).anonymous_named_1.h = 6;
    (*o.borrow_mut()).anon_2.e = 7;
    (*o.borrow_mut()).anon_2.f = 8;
    (*o.borrow_mut()).anon_5.i = 9;
    (*o.borrow_mut()).anon_5.inner_named.j = 10;
    (*o.borrow_mut()).anon_5.anon_4.k = 11;
    assert!(({ (*o.borrow()).named.a } == 1));
    assert!(({ (*o.borrow()).named.b } == 2));
    assert!(({ (*o.borrow()).anonymous_named_0.c } == 3));
    assert!(({ (*o.borrow()).anonymous_named_0.d } == 4));
    assert!(({ (*o.borrow()).anonymous_named_1.g } == 5));
    assert!(({ (*o.borrow()).anonymous_named_1.h } == 6));
    assert!(({ (*o.borrow()).anon_2.e } == 7));
    assert!(({ (*o.borrow()).anon_2.f } == 8));
    assert!(({ (*o.borrow()).anon_5.i } == 9));
    assert!(({ (*o.borrow()).anon_5.inner_named.j } == 10));
    assert!(({ (*o.borrow()).anon_5.anon_4.k } == 11));
    let s: Value<anon_6> = Rc::new(RefCell::new(<anon_6>::default()));
    (*s.borrow_mut()).x = 1;
    (*s.borrow_mut()).z = 2;
    assert!(
        ({
            (*s.borrow_mut()).x = 1;
            { (*s.borrow()).x }
        } != 0)
    );
    assert!(
        ({
            (*s.borrow_mut()).z = 2;
            { (*s.borrow()).z }
        } != 0)
    );
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_6 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub z: i32,
}
pub fn __cpp2rust_init_globals() {}
