extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn basic_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 0 => {
                r = 10;
                break 'switch;
            }
            v if v == 1 => {
                r = 20;
                break 'switch;
            }
            v if v == 2 => {
                r = 30;
                break 'switch;
            }
            _ => {
                r = 40;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn stacked_1(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                r = 100;
                break 'switch;
            }
            v if v == 4 || v == 5 => {
                r = 200;
                break 'switch;
            }
            _ => {
                r = 300;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn no_default_2(mut x: i32) -> i32 {
    let mut r: i32 = -1_i32;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 7 => {
                r = 1;
                break 'switch;
            }
            v if v == 8 => {
                r = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return r;
}
pub unsafe fn only_default_3(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            _ => {
                r = 42;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn default_middle_4(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                r = 1;
                break 'switch;
            }
            v if v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 99;
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
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            basic_0(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            basic_0(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            basic_0(_x)
        }) == (40))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            stacked_1(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            stacked_1(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            stacked_1(_x)
        }) == (200))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            stacked_1(_x)
        }) == (300))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 7;
            no_default_2(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 42;
            no_default_2(_x)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            only_default_3(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            default_middle_4(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            default_middle_4(_x)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            default_middle_4(_x)
        }) == (99))
    );
    return 0;
}
