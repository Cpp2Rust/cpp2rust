extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn classify_0(mut x: Ptr<i32>) -> i32 {
    {
        return (x.read());
    }
    return 1;
}
pub fn classify_1(mut x: i64) -> i32 {
    {
        {
            return 2;
        }
    }
    return 1;
}
pub fn classify_2(mut x: i32) -> i32 {
    {
        {}
    }
    return 1;
}
pub fn keep_both_3(mut x: i32) -> i32 {
    {
        return (x + 1);
    }
    panic!("ub: non-void function does not return a value")
}
pub fn widen_4(mut v: u32) -> u64 {
    {}
    return (v as u64);
}
pub fn widen_5(mut v: u64) -> u64 {
    {
        let mut hi: u64 = v;
        hi <<= 32;
        return hi;
    }
    return v;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(7));
    assert!((({ classify_0((v.as_pointer()),) }) == 7));
    assert!((({ classify_1(1_i64,) }) == 2));
    assert!((({ classify_2(1,) }) == 1));
    assert!((({ keep_both_3(1,) }) == 2));
    assert!((({ widen_4(1_u32,) }) == 1_u64));
    assert!((({ widen_5(1_u64,) }) == 4294967296_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
