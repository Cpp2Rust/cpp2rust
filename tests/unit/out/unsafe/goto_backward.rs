extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn retry_0(mut n: i32) -> i32 {
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
            if ((((count) < (3)) as i32) != 0) {
                goto!('again);
            }
            return acc;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _n: i32 = 4;
            retry_0(_n)
        }) == (12)) as i32)
            != 0)
    );
    return 0;
}
