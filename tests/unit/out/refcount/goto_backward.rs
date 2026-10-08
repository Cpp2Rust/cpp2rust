extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn retry_0(mut n: i32) -> i32 {
    let mut count: i32 = 0_i32;
    let mut acc: i32 = 0_i32;
    goto_block!({
        '__entry: {
            count = 0;
            acc = 0;
        }
        'again: {
            count += 1;
            acc += n;
            if (((count < 3) as i32) != 0) {
                goto!('again);
            }
            return acc;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ retry_0(4,) }) == 12) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
