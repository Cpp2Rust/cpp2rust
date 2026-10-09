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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut negate: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |x: i32| -> i32 {
            return -x;
        },
        |x: f64| -> f64 {
            return -x;
        }
    );
    let mut fi: Option<unsafe fn(i32) -> i32> = Some(|x: i32| -> i32 {
        return -x;
    });
    let mut fd: Option<unsafe fn(f64) -> f64> = Some(|x: f64| -> f64 {
        return -x;
    });
    assert!(((unsafe { (fi).unwrap()(3,) }) == (-3_i32)));
    assert!(((unsafe { (fd).unwrap()(1.5_f64,) }) == (-1.5_f64)));
    let mut square: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |x: i32| -> i32 {
            return ((x) * (x));
        },
        |x: f64| -> f64 {
            return ((x) * (x));
        }
    );
    let mut si: Option<unsafe fn(i32) -> i32> = Some(|x: i32| -> i32 {
        return ((x) * (x));
    });
    let mut sd: Option<unsafe fn(f64) -> f64> = Some(|x: f64| -> f64 {
        return ((x) * (x));
    });
    assert!(((unsafe { (si).unwrap()(3,) }) == (9)));
    assert!(((unsafe { (sd).unwrap()(1.5_f64,) }) == (2.25_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
