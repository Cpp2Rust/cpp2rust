extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Code = u32;
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
thread_local!(
    pub static side_effect_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn observe_1(mut v: i32) -> i32 {
    (*side_effect_0.with(Value::clone).borrow_mut()).prefix_inc();
    return v;
}
pub fn returns_one_2() -> i32 {
    return 1;
}
pub fn returns_zero_3() -> i32 {
    return 0;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let storage: Value<i32> = Rc::new(RefCell::new(7));
    let mut p: Ptr<i32> = (storage.as_pointer());
    let mut np: Ptr<i32> = Ptr::<i32>::null();
    let mut u: u32 = 4_u32;
    let mut code: Code = Code_CODE_OK;
    if ((((n != 0) && (!(p).is_null())) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((n != 0) && (!(np).is_null())) as i32) != 0) {
        assert!((0 != 0));
    }
    if ((((zero != 0) || (!(p).is_null())) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((zero != 0) || (!(np).is_null())) as i32) != 0) {
        assert!((0 != 0));
    }
    if ((((((((((n != 0) && (u != 0)) as i32) != 0) && (!(p).is_null())) as i32) != 0)
        && ((((code as u32) == ((Code_CODE_OK as i32) as u32)) as i32) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    side_effect_0.with(|rc| *rc.borrow_mut() = 0);
    if ((((zero != 0) && (({ observe_1(1) }) != 0)) as i32) != 0) {
        assert!((0 != 0));
    }
    assert!((((side_effect_0.with(|rc| *rc.borrow()) == 0) as i32) != 0));
    if ((((n != 0) || (({ observe_1(1) }) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    assert!((((side_effect_0.with(|rc| *rc.borrow()) == 0) as i32) != 0));
    let mut x: i32 = 5;
    let mut y: i32 = 3;
    let mut flags: u32 = 2_u32;
    if ((((((x > y) as i32) != 0) || ((flags & 1_u32) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x < y) as i32) != 0) || ((flags & 1_u32) != 0)) as i32) != 0) {
        assert!((0 != 0));
    }
    let mut a: u32 = 1_u32;
    let mut b: u32 = 2_u32;
    let mut c: u32 = 3_u32;
    if ((((((a != c) as i32) != 0) && (((b != c) as i32) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    let mut s: i32 = -1_i32;
    if ((((((!((p).is_null())) as i32) != 0) && (((s < 0) as i32) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    let mut k: u32 = 2_u32;
    let mut done: bool = (0 != 0);
    if ((((((k > 1_u32) as i32) != 0) || (!(done))) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x > y) as i32) != 0) || ((flags & 4_u32) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    let mut ull: u64 = 7_u64;
    if ((((((!((p).is_null())) as i32) != 0) && (ull != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x > y) as i32) != 0) && (ull != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    let mut mask: i64 = ((1_i64 << 4) | (1_i64 << 5));
    let mut bits: i64 = (1_i64 << 4);
    if ((((((n != 0) as i32) != 0) && ((bits & mask) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((n != 0) as i32) != 0) || ((bits & 256_i64) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    let mut cp: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hi");
    let mut cnp: Ptr<i8> = Ptr::<i8>::null();
    if ((((((x > y) as i32) != 0) && (!(cp).is_null())) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x < y) as i32) != 0) || (!(cnp).is_null())) as i32) != 0) {
        assert!((0 != 0));
    }
    if ((((((x > y) as i32) != 0) && ((((n != 0) && (!(cp).is_null())) as i32) != 0)) as i32) != 0)
    {
        assert!((1 != 0));
    }
    if ((((((x > y) as i32) != 0) && (({ returns_one_2() }) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x > y) as i32) != 0) && (!(({ returns_zero_3() }) != 0))) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x < y) as i32) != 0) || (({ returns_one_2() }) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    if ((((((x < y) as i32) != 0) || (!(({ returns_one_2() }) != 0))) as i32) != 0) {
        assert!((0 != 0));
    }
    if (((((((((!((p).is_null())) as i32) != 0) && (({ returns_one_2() }) != 0)) as i32) != 0)
        && (((n != 0) as i32) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = side_effect_0.with(|_| ());
}
