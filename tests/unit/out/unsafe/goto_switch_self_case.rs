extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sm_0(mut n: i32) -> i32 {
    let mut steps: i32 = 0;
    switch!(match n {
        __v if __v == 0 => 'target: {
            steps += 1;
            break;
        }
        __v if __v == 1 => {
            steps += 10;
            goto!('target);
        }
        _ => {
            steps = -1_i32;
            break;
        }
    });
    return steps;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { sm_0(0,) }) == (1)) as i32) != 0));
    assert!(((((unsafe { sm_0(1,) }) == (11)) as i32) != 0));
    assert!(((((unsafe { sm_0(7,) }) == (-1_i32)) as i32) != 0));
    return 0;
}
