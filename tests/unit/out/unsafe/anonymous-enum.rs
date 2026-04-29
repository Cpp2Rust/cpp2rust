extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_enum_3 {
    #[default]
    FIRST_A = 0,
    FIRST_B = 1,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_enum_11 {
    #[default]
    SECOND_A = 0,
    SECOND_B = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub a: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    enum anon_enum_18 {
        #[default]
        THIRD_A = 0,
        THIRD_B = 1,
    };
    assert!(((anon_enum_3::FIRST_A as i32) == (0)));
    assert!(((anon_enum_3::FIRST_B as i32) == (1)));
    assert!(((anon_enum_11::SECOND_A as i32) == (0)));
    assert!(((anon_enum_11::SECOND_B as i32) == (1)));
    assert!(((anon_enum_18::THIRD_A as i32) == (0)));
    assert!(((anon_enum_18::THIRD_B as i32) == (1)));
    return 0;
}
