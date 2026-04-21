extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn foo_1(mut x: f64) -> f64 {
    return x;
}
pub unsafe fn bar_2(mut p: *mut i32, mut flag: bool) -> *mut i32 {
    return if flag { p } else { Default::default() };
}
pub unsafe fn bar_3(mut p: *mut f64, mut flag: bool) -> *mut f64 {
    return if flag { p } else { Default::default() };
}
pub unsafe fn func_4(mut x1: i32, mut x2: i32, mut x3: i32) -> i32 {
    return (((x1) + (x2)) + (x3));
}
pub unsafe fn func_5(mut x1: f64, mut x2: i32, mut x3: f64) -> i32 {
    return ((((x1) + (x2 as f64)) + (x3)) as i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    let mut y: f64 = (x as f64);
    return ((((((((unsafe {
        let _x: i32 = x;
        foo_0(_x)
    }) as f64)
        + (unsafe {
            let _x: f64 = y;
            foo_1(_x)
        }))
        + ((*(unsafe {
            let _p: *mut i32 = (&mut x as *mut i32);
            let _flag: bool = true;
            bar_2(_p, _flag)
        })) as f64))
        + (*(unsafe {
            let _p: *mut f64 = (&mut y as *mut f64);
            let _flag: bool = true;
            bar_3(_p, _flag)
        })))
        + ((unsafe {
            let _x1: i32 = 1;
            let _x2: i32 = 2;
            let _x3: i32 = 3;
            func_4(_x1, _x2, _x3)
        }) as f64))
        + ((unsafe {
            let _x1: f64 = 2.0E+0;
            let _x2: i32 = x;
            let _x3: f64 = y;
            func_5(_x1, _x2, _x3)
        }) as f64)) as i32);
}
