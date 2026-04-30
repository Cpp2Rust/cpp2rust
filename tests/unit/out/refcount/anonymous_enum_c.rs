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
impl ByteRepr for S {}
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
#[derive(Default)]
pub struct WithAnonField {
    pub a: Value<i32>,
    pub field: Value<anon_enum_24>,
}
impl ByteRepr for WithAnonField {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    enum anon_enum_31 {
        #[default]
        THIRD_A = 0,
        THIRD_B = 1,
    };
    assert!((anon_enum_3::FIRST_A != anon_enum_3::FIRST_B));
    assert!((anon_enum_11::SECOND_A != anon_enum_11::SECOND_B));
    assert!((anon_enum_31::THIRD_A != anon_enum_31::THIRD_B));
    let td: Value<TdEnum> = Rc::new(RefCell::new((TdEnum::TD_A as TdEnum)));
    assert!((((*td.borrow()) as u32) == (TdEnum::TD_A as u32)));
    (*td.borrow_mut()) = (TdEnum::TD_B as TdEnum);
    assert!((((*td.borrow()) as u32) == (TdEnum::TD_B as u32)));
    let w: Value<WithAnonField> = <Value<WithAnonField>>::default();
    (*(*w.borrow()).field.borrow_mut()) = (anon_enum_24::FIELD_A as anon_enum_24);
    assert!((((*(*w.borrow()).field.borrow()) as u32) == (anon_enum_24::FIELD_A as u32)));
    (*(*w.borrow()).field.borrow_mut()) = (anon_enum_24::FIELD_B as anon_enum_24);
    assert!((((*(*w.borrow()).field.borrow()) as u32) == (anon_enum_24::FIELD_B as u32)));
    return 0;
}
