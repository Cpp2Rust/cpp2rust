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
    return ((unsafe {
        let _x: f64 = x;
        let _y: f64 = y;
        f2_0(_x, _y)
    }) + (z));
}
pub unsafe fn f1_2(mut x: f64, mut y: f64) -> f64 {
    let mut z1: f64 = (unsafe {
        let _x: f64 = x;
        let _y: f64 = y;
        f2_0(_x, _y)
    });
    if ((unsafe {
        let _x: f64 = z1;
        let _y: f64 = y;
        f2_0(_x, _y)
    }) < (0_f64))
    {
        let mut z2: f64 = -(unsafe {
            let _x: f64 = z1;
            let _y: f64 = (unsafe {
                let _x: f64 = x;
                let _y: f64 = y;
                f2_0(_x, _y)
            });
            let _z: f64 = y;
            f3_1(_x, _y, _z)
        });
        return (unsafe {
            let _x: f64 = (unsafe {
                let _x: f64 = z2;
                let _y: f64 = (unsafe {
                    let _x: f64 = z1;
                    let _y: f64 = z2;
                    let _z: f64 = x;
                    f3_1(_x, _y, _z)
                });
                f2_0(_x, _y)
            });
            let _y: f64 = y;
            f2_0(_x, _y)
        });
    }
    return (unsafe {
        let _x: f64 = z1;
        let _y: f64 = x;
        f2_0(_x, _y)
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return ((unsafe {
        let _x: f64 = 1.0E+0;
        let _y: f64 = 2.0E+0;
        f1_2(_x, _y)
    }) as i32);
}
