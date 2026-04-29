extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
#[derive(Default)]
pub struct S {
    pub a: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
        };
        this
    }
}
impl ByteRepr for S {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    enum anon_enum_18 {
        #[default]
        THIRD_A = 0,
        THIRD_B = 1,
    };
    assert!(((anon_enum_3::FIRST_A as i32) == 0));
    assert!(((anon_enum_3::FIRST_B as i32) == 1));
    assert!(((anon_enum_11::SECOND_A as i32) == 0));
    assert!(((anon_enum_11::SECOND_B as i32) == 1));
    assert!(((anon_enum_18::THIRD_A as i32) == 0));
    assert!(((anon_enum_18::THIRD_B as i32) == 1));
    return 0;
}
