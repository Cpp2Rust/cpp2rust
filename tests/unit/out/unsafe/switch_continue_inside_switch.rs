extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn continue_inside_switch_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            let __match_cond = i;
            match __match_cond {
                v if v == 0 || v == 2 || v == 4 => {
                    i.prefix_inc();
                    continue 'loop_;
                }
                _ => {
                    r += i;
                    break 'switch;
                }
            }
        };
        r += 1000;
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
    assert!(
        ((unsafe {
            let _n: i32 = 6;
            continue_inside_switch_0(_n)
        }) == ((((1) + (3)) + (5)) + ((3) * (1000))))
    );
    return 0;
}
