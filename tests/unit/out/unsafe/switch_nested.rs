extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn nested_0(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = a;
        match __match_cond {
            v if v == 1 => {
                'switch: {
                    let __match_cond = b;
                    match __match_cond {
                        v if v == 10 => {
                            r = 11;
                            break 'switch;
                        }
                        v if v == 20 => {
                            r = 12;
                            break 'switch;
                        }
                        _ => {
                            r = 13;
                            break 'switch;
                        }
                    }
                };
                r += 1;
                break;
            }
            v if v == 2 => {
                r = 2;
                break;
            }
            _ => {
                r = -1_i32;
                break;
            }
        }
    };
    return r;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _a: i32 = 1;
            let _b: i32 = 10;
            nested_0(_a, _b)
        }) == (12))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 1;
            let _b: i32 = 99;
            nested_0(_a, _b)
        }) == (14))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 2;
            let _b: i32 = 0;
            nested_0(_a, _b)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 3;
            let _b: i32 = 3;
            nested_0(_a, _b)
        }) == (-1_i32))
    );
    return 0;
}
