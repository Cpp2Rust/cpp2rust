extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_char_0(mut c: ::core::ffi::c_char) -> i32 {
    'switch: {
        let __match_cond = (c as i32);
        match __match_cond {
            __v if __v == (('a' as ::core::ffi::c_char) as i32) => {
                return 1;
            }
            __v if __v == (('b' as ::core::ffi::c_char) as i32) => {
                return 2;
            }
            __v if __v == (('\n' as ::core::ffi::c_char) as i32) => {
                return 3;
            }
            __v if __v == (('\0' as ::core::ffi::c_char) as i32) => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Color {
    #[default]
    kRed = 0,
    kGreen = 1,
    kBlue = 2,
}
impl From<i32> for Color {
    fn from(n: i32) -> Color {
        match n {
            0 => Color::kRed,
            1 => Color::kGreen,
            2 => Color::kBlue,
            _ => panic!("invalid Color value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Color);
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { switch_char_0(('a' as ::core::ffi::c_char),) }) == (1)));
    assert!(((unsafe { switch_char_0(('b' as ::core::ffi::c_char),) }) == (2)));
    assert!(((unsafe { switch_char_0(('\n' as ::core::ffi::c_char),) }) == (3)));
    assert!(((unsafe { switch_char_0(('\0' as ::core::ffi::c_char),) }) == (4)));
    assert!(((unsafe { switch_char_0(('z' as ::core::ffi::c_char),) }) == (0)));
    return 0;
}
