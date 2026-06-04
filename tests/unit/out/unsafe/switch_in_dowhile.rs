extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_in_dowhile_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let mut __do_while = true;
    'loop_: while __do_while || ((i) < (n)) {
        __do_while = false;
        'switch: {
            let __match_cond = i;
            match __match_cond {
                __v if __v == 0 => {
                    r += 1;
                    break 'switch;
                }
                __v if __v == 1 => {
                    r += 10;
                    break 'switch;
                }
                _ => {
                    r += 100;
                    break 'switch;
                }
            }
        };
        i.prefix_inc();
    }
    return r;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { switch_in_dowhile_0(1,) }) == (1)));
    assert!(((unsafe { switch_in_dowhile_0(3,) }) == (((1) + (10)) + (100))));
    return 0;
}
