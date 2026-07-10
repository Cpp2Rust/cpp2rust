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
    return if flag { p } else { std::ptr::null_mut() };
}
pub unsafe fn bar_3(mut p: *mut f64, mut flag: bool) -> *mut f64 {
    return if flag { p } else { std::ptr::null_mut() };
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
    return ((((((((unsafe { foo_0(x) }) as f64) + (unsafe { foo_1(y) }))
        + ((*(unsafe { bar_2((&mut x as *mut i32), true) })) as f64))
        + (*(unsafe { bar_3((&mut y as *mut f64), true) })))
        + ((unsafe { func_4(1, 2, 3) }) as f64))
        + ((unsafe { func_5(2.0E+0, x, y) }) as f64)) as i32);
}
