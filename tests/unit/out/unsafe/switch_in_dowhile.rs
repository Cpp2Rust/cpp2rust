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
    'loop_: loop {
        'switch: {
            let __match_cond = i;
            match __match_cond {
                v if v == 0 => {
                    r += 1;
                    break 'switch;
                }
                v if v == 1 => {
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
        if !((i) < (n)) {
            break;
        }
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
            let _n: i32 = 1;
            switch_in_dowhile_0(_n)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 3;
            switch_in_dowhile_0(_n)
        }) == (((1) + (10)) + (100)))
    );
    return 0;
}
