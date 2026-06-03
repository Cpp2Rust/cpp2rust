extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn agg_0(mut n: i32) -> i32 {
    let mut buf40: [u8; 40] = [0_u8; 40];
    let mut buf256: [u8; 256] = [0_u8; 256];
    let mut arr64: [i32; 64] = [0_i32; 64];
    let mut longs: [i64; 33] = [0_i64; 33];
    let mut p: Point = <Point>::default();
    let mut ptr: *mut i32 = std::ptr::null_mut();
    let mut fp: Option<unsafe fn(i32) -> i32> = None;
    let mut file: *mut ::libc::FILE = std::ptr::null_mut();
    let mut total: i32 = 0_i32;
    goto_block!({
        '__entry: {
            total = 0;
            if ((((n) < (0)) as i32) != 0) {
                goto!('out);
            }
            total = 1;
        }
        'out: {
            return total;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _n: i32 = -1_i32;
            agg_0(_n)
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 1;
            agg_0(_n)
        }) == (1)) as i32)
            != 0)
    );
    return 0;
}
