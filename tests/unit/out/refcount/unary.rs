extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 0;
    let mut a: [i32; 3] = [0, 1, 2];
    'loop_: while (x < 3) {
        a[(x.postfix_inc()) as usize].prefix_inc();
    }
    let mut out: i32 = 0;
    'loop_: while (x != 0) {
        out += a[(x.prefix_dec()) as usize];
    }
    out.postfix_inc();
    let mut x2: i32 = out.prefix_dec();
    out.prefix_inc();
    let mut x3: i32 = out.postfix_dec();
    assert!((((out.postfix_inc() + x2) + x3) == 19));
    let mut n: i32 = x2;
    let mut d: f64 = 1.5_f64;
    assert!((n == x2));
    assert!((d == 1.5_f64));
    assert!(((n + n) == (2 * x2)));
    assert!((a[(0) as usize] == a[(0) as usize]));
    let mut wide: i128 = 5_i128;
    let mut w1: i128 = wide.postfix_inc();
    let mut w2: i128 = wide.prefix_inc();
    let mut w3: i128 = wide.postfix_dec();
    let mut w4: i128 = wide.prefix_dec();
    assert!((((w1 == 5_i128) && (w2 == 7_i128)) && (w3 == 7_i128)) && (w4 == 5_i128));
    let mut uwide: u128 = 0_u128;
    uwide.prefix_dec();
    uwide.postfix_inc();
    assert!((uwide == 0_u128));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
