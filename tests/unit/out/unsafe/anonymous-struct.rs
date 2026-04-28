extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Outer_Named {
    pub a: i32,
    pub b: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_0 {
    pub c: i32,
    pub d: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_1 {
    pub g: i32,
    pub h: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_2 {
    pub e: i32,
    pub f: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_3_anon_0 {
    pub j: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_3_anon_1 {
    pub k: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_3 {
    pub i: i32,
    pub inner_named: Outer_anon_3_anon_0,
    pub anon_1: Outer_anon_3_anon_1,
}
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub named: Outer_Named,
    pub anonymous_named_0: Outer_anon_0,
    pub anonymous_named_1: Outer_anon_1,
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
        named: Outer_Named { a: 0_i32, b: 0_i32 },
        anonymous_named_0: Outer_anon_0 { c: 0_i32, d: 0_i32 },
        anonymous_named_1: Outer_anon_1 { g: 0_i32, h: 0_i32 },
        anon_2: Outer_anon_2 { e: 0_i32, f: 0_i32 },
        anon_3: Outer_anon_3 {
            i: 0_i32,
            inner_named: Outer_anon_3_anon_0 { j: 0_i32 },
            anon_1: Outer_anon_3_anon_1 { k: 0_i32 },
        },
    };
    o.named.a = 1;
    o.named.b = 2;
    o.anonymous_named_0.c = 3;
    o.anonymous_named_0.d = 4;
    o.anonymous_named_1.g = 5;
    o.anonymous_named_1.h = 6;
    o.anon_2.e = 7;
    o.anon_2.f = 8;
    o.anon_3.i = 9;
    o.anon_3.inner_named.j = 10;
    o.anon_3.anon_1.k = 11;
    assert!(((o.named.a) == (1)));
    assert!(((o.named.b) == (2)));
    assert!(((o.anonymous_named_0.c) == (3)));
    assert!(((o.anonymous_named_0.d) == (4)));
    assert!(((o.anonymous_named_1.g) == (5)));
    assert!(((o.anonymous_named_1.h) == (6)));
    assert!(((o.anon_2.e) == (7)));
    assert!(((o.anon_2.f) == (8)));
    assert!(((o.anon_3.i) == (9)));
    assert!(((o.anon_3.inner_named.j) == (10)));
    assert!(((o.anon_3.anon_1.k) == (11)));
    return 0;
}
