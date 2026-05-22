extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i64 = 0_i64;
    assert!(
        ((!{
            let (val, ovf) = 3_i64.overflowing_mul(7_i64);
            *(&mut a as *mut i64) = val;
            ovf
        } as i32)
            != 0)
    );
    assert!(((((a) == (21_i64)) as i32) != 0));
    let mut b: i64 = 0_i64;
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        *(&mut b as *mut i64) = val;
        ovf
    });
    let mut c: i64 = 0_i64;
    assert!(
        ((!{
            let (val, ovf) = 1000_i64.overflowing_mul(1000_i64);
            *(&mut c as *mut i64) = val;
            ovf
        } as i32)
            != 0)
    );
    assert!(((((c) == (1000000_i64)) as i32) != 0));
    let mut d: i64 = 0_i64;
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        *(&mut d as *mut i64) = val;
        ovf
    });
    return 0;
}
