extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn for_in_switch_continue_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = n;
        match __match_cond {
            v if v == 0 => {
                let mut i: i32 = 0;
                'loop_: while ((i) < (5)) {
                    if (((i) % (2)) == (0)) {
                        i.prefix_inc();
                        continue 'loop_;
                    }
                    r += i;
                    i.prefix_inc();
                }
                break 'switch;
            }
            _ => {
                r = -1_i32;
                break 'switch;
            }
        }
    };
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
            let _n: i32 = 0;
            for_in_switch_continue_0(_n)
        }) == (4))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 99;
            for_in_switch_continue_0(_n)
        }) == (-1_i32))
    );
    return 0;
}
