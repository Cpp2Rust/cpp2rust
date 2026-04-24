extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_on_assignment_0(mut x: i32) -> i32 {
    let mut y: i32 = 0;
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = {
            y = ((x) + (1));
            y
        };
        match __match_cond {
            v if v == 1 => {
                r = 10;
                break 'switch;
            }
            v if v == 2 => {
                r = 20;
                break 'switch;
            }
            _ => {
                r = y;
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
            let _x: i32 = 0;
            switch_on_assignment_0(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            switch_on_assignment_0(_x)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            switch_on_assignment_0(_x)
        }) == (10))
    );
    return 0;
}
