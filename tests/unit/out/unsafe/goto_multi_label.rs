extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if ((((n) < (0)) as i32) != 0) {
                goto!('error);
            }
            if ((((n) == (0)) as i32) != 0) {
                goto!('out);
            }
            ret = n;
            goto!('out);
        }
        'error: {
            ret = -1_i32;
        }
        'out: {
            return ret;
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
    assert!(((((unsafe { classify_0(5,) }) == (5)) as i32) != 0));
    assert!(((((unsafe { classify_0(0,) }) == (0)) as i32) != 0));
    assert!(
        ((((unsafe {
            let _n: i32 = -2_i32;
            classify_0(_n)
        }) == (-1_i32)) as i32)
            != 0)
    );
    return 0;
}
