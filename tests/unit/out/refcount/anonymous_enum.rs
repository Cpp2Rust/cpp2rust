extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_FIRST_A: anon_0 = 0;
pub const anon_0_FIRST_B: anon_0 = 1;
pub type anon_1 = u32;
pub const anon_1_SECOND_A: anon_1 = 0;
pub const anon_1_SECOND_B: anon_1 = 1;
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
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub type TdEnum = u32;
pub const TdEnum_TD_A: TdEnum = 0;
pub const TdEnum_TD_B: TdEnum = 1;
pub type anon_2 = u32;
pub const anon_2_FIELD_A: anon_2 = 0;
pub const anon_2_FIELD_B: anon_2 = 1;
#[derive(Default)]
pub struct WithAnonField {
    pub a: Value<i32>,
    pub field: Value<anon_2>,
}
impl Clone for WithAnonField {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            field: Rc::new(RefCell::new((*self.field.borrow()))),
        };
        this
    }
}
impl ByteRepr for WithAnonField {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.field.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            field: Rc::new(RefCell::new(<anon_2>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    pub type anon_3 = u32;
    pub const anon_3_THIRD_A: anon_3 = 0;
    pub const anon_3_THIRD_B: anon_3 = 1;;
    assert!(((anon_0_FIRST_A as i32) != (anon_0_FIRST_B as i32)));
    assert!(((anon_1_SECOND_A as i32) != (anon_1_SECOND_B as i32)));
    assert!(((anon_3_THIRD_A as i32) != (anon_3_THIRD_B as i32)));
    let td: Value<TdEnum> = Rc::new(RefCell::new(TdEnum_TD_A));
    assert!((((*td.borrow()) as i32) == (TdEnum_TD_A as i32)));
    (*td.borrow_mut()) = TdEnum_TD_B;
    assert!((((*td.borrow()) as i32) == (TdEnum_TD_B as i32)));
    let w: Value<WithAnonField> = Rc::new(RefCell::new(<WithAnonField>::default()));
    (*(*w.borrow()).field.borrow_mut()) = anon_2_FIELD_A;
    assert!((((*(*w.borrow()).field.borrow()) as i32) == (anon_2_FIELD_A as i32)));
    (*(*w.borrow()).field.borrow_mut()) = anon_2_FIELD_B;
    assert!((((*(*w.borrow()).field.borrow()) as i32) == (anon_2_FIELD_B as i32)));
    return 0;
}
