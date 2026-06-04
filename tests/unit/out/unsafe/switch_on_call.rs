extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut v: i32) -> i32 {
    return ((v) * (2));
}
pub unsafe fn switch_on_call_1(mut x: i32) -> i32 {
    'switch: {
        let __match_cond = (unsafe {
            let _v: i32 = x;
            double_it_0(_v)
        });
        match __match_cond {
            __v if __v == 0 => {
                return 100;
            }
            __v if __v == 2 => {
                return 200;
            }
            __v if __v == 4 => {
                return 400;
            }
            _ => {
                return -1_i32;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { switch_on_call_1(0,) }) == (100)));
    assert!(((unsafe { switch_on_call_1(1,) }) == (200)));
    assert!(((unsafe { switch_on_call_1(2,) }) == (400)));
    assert!(((unsafe { switch_on_call_1(99,) }) == (-1_i32)));
    return 0;
}
