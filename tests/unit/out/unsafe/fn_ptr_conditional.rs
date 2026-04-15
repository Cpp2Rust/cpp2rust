extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut x: i32) -> i32 {
    return ((x) + (1));
}
pub unsafe fn dec_1(mut x: i32) -> i32 {
    return ((x) - (1));
}
pub unsafe fn identity_2(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn pick_3(mut mode: i32) -> Option<unsafe fn(i32) -> i32> {
    return if ((mode) > (0)) {
        Some(inc_0)
    } else {
        if ((mode) < (0)) {
            Some(dec_1)
        } else {
            Some(identity_2)
        }
    };
}
pub unsafe fn apply_4(mut fn_: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
    let mut actual: Option<unsafe fn(i32) -> i32> = if !(fn_).is_none() {
        fn_
    } else {
        Some(identity_2)
    };
    return (unsafe {
        let _arg0: i32 = x;
        (actual).unwrap()(_arg0)
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            (unsafe {
                let _mode: i32 = 1;
                pick_3(_mode)
            })
            .unwrap()(_arg0)
        }) == (11))
    );
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            (unsafe {
                let _mode: i32 = -1_i32;
                pick_3(_mode)
            })
            .unwrap()(_arg0)
        }) == (9))
    );
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            (unsafe {
                let _mode: i32 = 0;
                pick_3(_mode)
            })
            .unwrap()(_arg0)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _fn: Option<unsafe fn(i32) -> i32> = Some(inc_0);
            let _x: i32 = 5;
            apply_4(_fn, _x)
        }) == (6))
    );
    assert!(
        ((unsafe {
            let _fn: Option<unsafe fn(i32) -> i32> = None;
            let _x: i32 = 5;
            apply_4(_fn, _x)
        }) == (5))
    );
    return 0;
}
