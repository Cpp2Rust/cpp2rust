extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn twice_0(v: *const i64) -> i64 {
    return ((*v) * (2_i64));
}
pub unsafe fn sum_1(p: *const Pair) -> i32 {
    return (((*p).first) + ((*p).second));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 1;
    let mut y: i32 = {
        x = 2;
        ((x) + (1))
    };
    assert!(((x) == (2)));
    assert!(((y) == (3)));
    let mut z: i32 = {
        {
            1;
            2
        };
        3
    };
    assert!(((z) == (3)));
    let mut counter: i32 = 0;
    let mut w: i32 = {
        {
            counter.postfix_inc();
            counter.postfix_inc()
        };
        counter
    };
    assert!(((counter) == (2)));
    assert!(((w) == (2)));
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    if {
        {
            a = 1;
            b = 2
        };
        (((a) + (b)) > (0))
    } {
        assert!(((a) == (1)));
        assert!(((b) == (2)));
    }
    let mut v1: Vec<i32> = vec![1, 2];
    let mut v2: Vec<i32> = {
        a = 5;
        (v1).clone()
    }
    .clone();
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    assert!(((a) == (5)));
    assert!(((v1.len()) == (2_usize)));
    assert!(((v2.len()) == (3_usize)));
    let mut p1: Pair = Pair {
        first: 1,
        second: 2,
    };
    let mut p2: Pair = {
        b = 6;
        p1
    };
    p2.first = 10;
    assert!(((p1.first) == (1)));
    assert!(((p2.first) == (10)));
    assert!(
        ((unsafe {
            let mut _v: i64 = (a as i64);
            twice_0(&mut _v)
        }) == (10_i64))
    );
    assert!(
        ((unsafe {
            sum_1(&{
                a = 7;
                p1
            })
        }) == (3))
    );
    assert!(((a) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
