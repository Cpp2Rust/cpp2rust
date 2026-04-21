extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fib_0(n: u64) -> u64 {
    let n: Value<u64> = Rc::new(RefCell::new(n));
    return if (((*n.borrow()) == 0_u64) || ((*n.borrow()) == 1_u64)) {
        (*n.borrow())
    } else {
        ({
            let _n: u64 = (*n.borrow()).wrapping_sub(1_u64);
            fib_0(_n)
        })
        .wrapping_add(
            ({
                let _n: u64 = (*n.borrow()).wrapping_sub(2_u64);
                fib_0(_n)
            }),
        )
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return (({
        let _n: u64 = 46_u64;
        fib_0(_n)
    }) as i32);
}
