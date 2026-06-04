extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sm_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            switch!(match n {
                __v if __v == 0 => {
                    ret += 1;
                }
                __v if __v == 1 => {
                    ret += 10;
                    goto!('out);
                }
                _ => {
                    ret += 100;
                    break;
                }
            });
            ret += 1000;
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
    assert!(((((unsafe { sm_0(0,) }) == (11)) as i32) != 0));
    assert!(((((unsafe { sm_0(1,) }) == (10)) as i32) != 0));
    assert!(((((unsafe { sm_0(9,) }) == (1100)) as i32) != 0));
    return 0;
}
