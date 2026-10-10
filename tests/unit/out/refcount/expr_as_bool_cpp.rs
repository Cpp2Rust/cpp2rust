extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_eq_0(mut rc: i32) -> i32 {
    return ((rc == -1_i32) as i32);
}
pub fn cmp_or_ptr_1(mut p: Ptr<i8>, mut q: Ptr<i8>) -> i32 {
    return (((!(p).is_null()) || (!(q).is_null())) as i32);
}
pub fn both_null_2(mut s1: Ptr<i8>, mut s2: Ptr<i8>) -> i32 {
    return ((((s1).is_null()) && ((s2).is_null())) as i32);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0_i32;
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while (({
        b = a;
        b
    }) != 0)
    {}
    if (a != 0) {}
    if (a == b) {}
    if (a < b) {}
    assert!((a == b));
    assert!(
        !(({
            a = b;
            a
        }) != 0)
    );
    let mut c: bool = false;
    c = ({
        a = b;
        a
    } != 0);
    c = (({
        b = a;
        b
    }) != 0);
    c = (a != 0);
    c = (a == b);
    c = (a < b);
    let mut x: i32 = 5;
    let mut y: i32 = 5;
    let mut eq: i32 = ((x == y) as i32);
    let mut lt: i32 = ((x < y) as i32);
    let mut neq: i32 = ((x != y) as i32);
    assert!((eq == 1));
    assert!((lt == 0));
    assert!((neq == 0));
    let mut p1: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hi");
    let mut p2: Ptr<i8> = Ptr::<i8>::null();
    let mut either: i32 = (((!(p1).is_null()) || (!(p2).is_null())) as i32);
    let mut both: i32 = (((!(p1).is_null()) && (!(p2).is_null())) as i32);
    assert!((either == 1));
    assert!((both == 0));
    assert!((({ cmp_eq_0(-1_i32,) }) == 1));
    assert!((({ cmp_eq_0(0,) }) == 0));
    assert!((({ cmp_or_ptr_1((p1).clone(), (p2).clone(),) }) == 1));
    assert!((({ cmp_or_ptr_1(Ptr::<i8>::null(), Ptr::<i8>::null(),) }) == 0));
    assert!((({ both_null_2(Ptr::<i8>::null(), Ptr::<i8>::null(),) }) == 1));
    assert!((({ both_null_2((p1).clone(), Ptr::<i8>::null(),) }) == 0));
    let mut d: f64 = 0.5_f64;
    let mut taken: i32 = 0;
    if (d != 0.0) {
        taken = 1;
    }
    assert!((taken == 1));
    'loop_: while (d != 0.0) {
        d = 0_f64;
    }
    assert!((d == 0_f64));
    let mut f: f32 = 2_f32;
    let mut fb: bool = (f != 0.0);
    let mut nf: bool = !(f != 0.0);
    assert!(fb);
    assert!(!(nf));
    assert!(!(0_f64 != 0.0));
    let mut fi: i32 = ((f != 0.0) as i32);
    assert!((fi == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
