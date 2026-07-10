extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f2_0(mut x: f64, mut y: f64) -> f64 {
    return ((x) - (y));
}
pub unsafe fn f3_1(mut x: f64, mut y: f64, mut z: f64) -> f64 {
    return ((unsafe { f2_0(x, y) }) + (z));
}
pub unsafe fn f1_2(mut x: f64, mut y: f64) -> f64 {
    let mut z1: f64 = (unsafe { f2_0(x, y) });
    if ((unsafe { f2_0(z1, y) }) < (0_f64)) {
        let mut z2: f64 = -(unsafe {
            let _y: f64 = (unsafe { f2_0(x, y) });
            let _z: f64 = y;
            f3_1(z1, _y, _z)
        });
        return (unsafe {
            f2_0(
                (unsafe {
                    let _x: f64 = z2;
                    let _y: f64 = (unsafe { f3_1(z1, z2, x) });
                    f2_0(_x, _y)
                }),
                y,
            )
        });
    }
    return (unsafe { f2_0(z1, x) });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return ((unsafe { f1_2(1.0E+0, 2.0E+0) }) as i32);
}
