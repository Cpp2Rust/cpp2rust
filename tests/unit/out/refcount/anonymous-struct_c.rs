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
pub struct Named {
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
pub struct anon_4 {
    #[offset(0)]
    pub j: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_5 {
    #[offset(0)]
    pub k: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct anon_3 {
    #[offset(0)]
    pub i: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner_named: anon_4,
    #[offset(8)]
    #[byte_size(4)]
    pub anon_5: anon_5,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(44)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub named: Named,
    #[offset(8)]
    #[byte_size(8)]
    pub anon0: anon_0,
    #[offset(16)]
    #[byte_size(8)]
    pub anon1: anon_1,
    #[offset(24)]
    #[byte_size(8)]
    pub anon_2: anon_2,
    #[offset(32)]
    #[byte_size(12)]
    pub anon_3: anon_3,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut o: Outer = Outer {
        named: Named { a: 0, b: 0_i32 },
        anon0: <anon_0>::default(),
        anon1: <anon_1>::default(),
        anon_2: <anon_2>::default(),
        anon_3: <anon_3>::default(),
    };
    o.named.a = 1;
    o.named.b = 2;
    o.anon0.c = 3;
    o.anon0.d = 4;
    o.anon1.g = 5;
    o.anon1.h = 6;
    o.anon_2.e = 7;
    o.anon_2.f = 8;
    o.anon_3.i = 9;
    o.anon_3.inner_named.j = 10;
    o.anon_3.anon_5.k = 11;
    assert!((((o.named.a == 1) as i32) != 0));
    assert!((((o.named.b == 2) as i32) != 0));
    assert!((((o.anon0.c == 3) as i32) != 0));
    assert!((((o.anon0.d == 4) as i32) != 0));
    assert!((((o.anon1.g == 5) as i32) != 0));
    assert!((((o.anon1.h == 6) as i32) != 0));
    assert!((((o.anon_2.e == 7) as i32) != 0));
    assert!((((o.anon_2.f == 8) as i32) != 0));
    assert!((((o.anon_3.i == 9) as i32) != 0));
    assert!((((o.anon_3.inner_named.j == 10) as i32) != 0));
    assert!((((o.anon_3.anon_5.k == 11) as i32) != 0));
    let mut s: anon_6 = <anon_6>::default();
    s.x = 1;
    s.z = 2;
    assert!(
        ({
            s.x = 1;
            s.x
        } != 0)
    );
    assert!(
        ({
            s.z = 2;
            s.z
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
