extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = u32;
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
pub unsafe fn switch_enum_0(mut c: Color) -> i32 {
    'switch: {
        let __match_cond = (c as i32);
        match __match_cond {
            __v if __v == (Color_kRed as i32) => {
                return 10;
            }
            __v if __v == (Color_kGreen as i32) => {
                return 20;
            }
            __v if __v == (Color_kBlue as i32) => {
                return 30;
            }
            _ => {}
        }
    };
    return -1_i32;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { switch_enum_0(Color_kRed,) }) == (10)));
    assert!(((unsafe { switch_enum_0(Color_kGreen,) }) == (20)));
    assert!(((unsafe { switch_enum_0(Color_kBlue,) }) == (30)));
    return 0;
}
