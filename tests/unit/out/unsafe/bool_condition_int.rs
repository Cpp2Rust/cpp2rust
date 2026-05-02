extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let mut u: u32 = 4_u32;
    let mut ul: u64 = 5_u64;
    let mut ll: i64 = 6_i64;
    let mut ch: u8 = ('a' as u8);
    if (n != 0) {
        assert!(true);
    }
    if (!(n != 0) as bool) {
        assert!(false);
    }
    if (zero != 0) {
        assert!(false);
    }
    if (!(zero != 0) as bool) {
        assert!(true);
    }
    if (u != 0) {
        assert!(true);
    }
    if (ul != 0) {
        assert!(true);
    }
    if (ll != 0) {
        assert!(true);
    }
    if (ch != 0) {
        assert!(true);
    }
    let mut loop_count: i32 = 0;
    let mut counter: i32 = 3;
    'loop_: while (counter != 0) {
        counter.prefix_dec();
        loop_count.prefix_inc();
    }
    assert!(((loop_count) == (3)));
    let mut i: i32 = 5;
    'loop_: while (i != 0) {
        loop_count.prefix_inc();
        i.prefix_dec();
    }
    assert!(((loop_count) == (8)));
    let mut t: i32 = if (n != 0) { 100 } else { 200 };
    assert!(((t) == (100)));
    let mut t2: i32 = if (zero != 0) { 100 } else { 200 };
    assert!(((t2) == (200)));
    let mut t7: i32 = ((!(n != 0) as bool) as i32);
    assert!(((t7) == (0)));
    let mut t8: i32 = ((!(zero != 0) as bool) as i32);
    assert!(((t8) == (1)));
    let mut b1: bool = (n != 0);
    assert!(b1);
    return 0;
}
