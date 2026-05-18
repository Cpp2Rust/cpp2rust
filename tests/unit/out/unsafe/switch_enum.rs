extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub unsafe fn switch_enum_0(mut c: Color) -> i32 {
    'switch: {
        let __match_cond = (c as i32);
        match __match_cond {
            v if v == (Color::kRed as i32) => {
                return 10;
            }
            v if v == (Color::kGreen as i32) => {
                return 20;
            }
            v if v == (Color::kBlue as i32) => {
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
    assert!(
        ((unsafe {
            let _c: Color = Color::kRed;
            switch_enum_0(_c)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _c: Color = Color::kGreen;
            switch_enum_0(_c)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _c: Color = Color::kBlue;
            switch_enum_0(_c)
        }) == (30))
    );
    return 0;
}
