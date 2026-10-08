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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub a: i32,
}
pub type TdEnum = u32;
pub const TdEnum_TD_A: TdEnum = 0;
pub const TdEnum_TD_B: TdEnum = 1;
pub type anon_2 = u32;
pub const anon_2_FIELD_A: anon_2 = 0;
pub const anon_2_FIELD_B: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct WithAnonField {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub field: anon_2,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((anon_0_FIRST_A as i32) != (anon_0_FIRST_B as i32)));
    assert!(((anon_1_SECOND_A as i32) != (anon_1_SECOND_B as i32)));
    assert!(((anon_3_THIRD_A as i32) != (anon_3_THIRD_B as i32)));
    let mut td: TdEnum = TdEnum_TD_A;
    assert!(((td as i32) == (TdEnum_TD_A as i32)));
    td = TdEnum_TD_B;
    assert!(((td as i32) == (TdEnum_TD_B as i32)));
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_2_FIELD_A;
    assert!(((w.field as i32) == (anon_2_FIELD_A as i32)));
    w.field = anon_2_FIELD_B;
    assert!(((w.field as i32) == (anon_2_FIELD_B as i32)));
    return 0;
}
pub type anon_3 = u32;
pub const anon_3_THIRD_A: anon_3 = 0;
pub const anon_3_THIRD_B: anon_3 = 1;
pub fn __cpp2rust_init_globals() {}
