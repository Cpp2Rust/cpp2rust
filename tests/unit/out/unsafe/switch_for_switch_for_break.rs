extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn for_switch_for_break_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            let __match_cond = i;
            match __match_cond {
                v if v == 1 => {
                    let mut j: i32 = 0;
                    'loop_: while ((j) < (10)) {
                        if ((j) == (2)) {
                            break;
                        }
                        r += 1;
                        j.prefix_inc();
                    }
                    r += 100;
                    break 'switch;
                }
                _ => {
                    r += 10;
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
    assert!(
        ((unsafe {
            let _n: i32 = 3;
            for_switch_for_break_0(_n)
        }) == (122))
    );
    return 0;
}
