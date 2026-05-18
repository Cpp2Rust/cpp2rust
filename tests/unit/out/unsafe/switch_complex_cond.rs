extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_complex_cond_0(mut p: *mut i32, mut bias: i32) -> i32 {
    'switch: {
        let __match_cond = ((*p) + (bias));
        match __match_cond {
            v if v == 0 => {
                return 1;
            }
            v if v == 5 => {
                return 2;
            }
            v if v == 10 => {
                return 3;
            }
            _ => {
                return 0;
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
    let mut p_val: i32 = 5;
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 0;
            switch_complex_cond_0(_p, _bias)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 5;
            switch_complex_cond_0(_p, _bias)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = -5_i32;
            switch_complex_cond_0(_p, _bias)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 99;
            switch_complex_cond_0(_p, _bias)
        }) == (0))
    );
    return 0;
}
