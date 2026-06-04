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
            __v if __v == 1 => {
                'switch: {
                    let __match_cond = b;
                    match __match_cond {
                        __v if __v == 10 => {
                            r = 11;
                            break 'switch;
                        }
                        __v if __v == 20 => {
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
                break 'switch;
            }
            __v if __v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = -1_i32;
                break 'switch;
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
    assert!(((unsafe { nested_0(1, 10,) }) == (12)));
    assert!(((unsafe { nested_0(1, 99,) }) == (14)));
    assert!(((unsafe { nested_0(2, 0,) }) == (2)));
    assert!(((unsafe { nested_0(3, 3,) }) == (-1_i32)));
    return 0;
}
