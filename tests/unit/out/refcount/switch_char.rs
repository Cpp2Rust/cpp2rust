extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_char_0(mut c: i8) -> i32 {
    'switch: {
        match { (c as i32) } {
            __v if __v == (('a' as i8) as i32) => {
                return 1;
            }
            __v if __v == (('b' as i8) as i32) => {
                return 2;
            }
            __v if __v == (('\n' as i8) as i32) => {
                return 3;
            }
            __v if __v == (('\0' as i8) as i32) => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub type Color = u32;
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ switch_char_0(('a' as i8),) }) == 1));
    assert!((({ switch_char_0(('b' as i8),) }) == 2));
    assert!((({ switch_char_0(('\n' as i8),) }) == 3));
    assert!((({ switch_char_0(('\0' as i8),) }) == 4));
    assert!((({ switch_char_0(('z' as i8),) }) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
