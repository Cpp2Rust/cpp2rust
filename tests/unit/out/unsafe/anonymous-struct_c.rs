extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Named {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_0 {
    pub c: i32,
    pub d: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_1 {
    pub g: i32,
    pub h: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_2 {
    pub e: i32,
    pub f: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_3_anon_0 {
    pub j: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_3_anon_1 {
    pub k: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_3 {
    pub i: i32,
    pub inner_named: Outer_anon_3_anon_0,
    pub anon_1: Outer_anon_3_anon_1,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub named: Named,
    pub anon0: Outer_anon_0,
    pub anon1: Outer_anon_1,
    pub anon_2: Outer_anon_2,
    pub anon_3: Outer_anon_3,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = Outer {
        named: Named { a: 0, b: 0_i32 },
        anon0: <Outer_anon_0>::default(),
        anon1: <Outer_anon_1>::default(),
        anon_2: <Outer_anon_2>::default(),
        anon_3: <Outer_anon_3>::default(),
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
    o.anon_3.anon_1.k = 11;
    assert!(((((o.named.a) == (1)) as i32) != 0));
    assert!(((((o.named.b) == (2)) as i32) != 0));
    assert!(((((o.anon0.c) == (3)) as i32) != 0));
    assert!(((((o.anon0.d) == (4)) as i32) != 0));
    assert!(((((o.anon1.g) == (5)) as i32) != 0));
    assert!(((((o.anon1.h) == (6)) as i32) != 0));
    assert!(((((o.anon_2.e) == (7)) as i32) != 0));
    assert!(((((o.anon_2.f) == (8)) as i32) != 0));
    assert!(((((o.anon_3.i) == (9)) as i32) != 0));
    assert!(((((o.anon_3.inner_named.j) == (10)) as i32) != 0));
    assert!(((((o.anon_3.anon_1.k) == (11)) as i32) != 0));
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct anon_0 {
        pub x: i32,
        pub z: i32,
    };
    let mut s: anon_0 = <anon_0>::default();
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
