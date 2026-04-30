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
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum TdEnum {
    #[default]
    TD_A = 0,
    TD_B = 1,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_enum_24 {
    #[default]
    FIELD_A = 0,
    FIELD_B = 1,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct WithAnonField {
    pub a: i32,
    pub field: anon_enum_24,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    enum anon_enum_31 {
        #[default]
        THIRD_A = 0,
        THIRD_B = 1,
    };
    assert!(((anon_enum_3::FIRST_A as i32) != (anon_enum_3::FIRST_B as i32)));
    assert!(((anon_enum_11::SECOND_A as i32) != (anon_enum_11::SECOND_B as i32)));
    assert!(((anon_enum_31::THIRD_A as i32) != (anon_enum_31::THIRD_B as i32)));
    let mut td: TdEnum = TdEnum::TD_A;
    assert!(((td as i32) == (TdEnum::TD_A as i32)));
    td = (TdEnum::TD_B).clone();
    assert!(((td as i32) == (TdEnum::TD_B as i32)));
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_enum_24::FIELD_A;
    assert!(((w.field as i32) == (anon_enum_24::FIELD_A as i32)));
    w.field = (anon_enum_24::FIELD_B).clone();
    assert!(((w.field as i32) == (anon_enum_24::FIELD_B as i32)));
    return 0;
}
