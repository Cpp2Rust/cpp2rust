extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn borrow_in_condition_and_in_body_0(mut x: i32) -> i32 {
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 0 => {}
            _ => {
                return ((x) + (1));
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
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            borrow_in_condition_and_in_body_0(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            borrow_in_condition_and_in_body_0(_x)
        }) == (2))
    );
    return 0;
}
