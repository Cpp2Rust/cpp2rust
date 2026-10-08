extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(mut t: Local_1) -> i32 {
    return (t.x as i32);
}
pub fn get_2(mut t: Local_3) -> i32 {
    return t.x;
}
pub fn get_4(mut t: Local_5) -> i32 {
    return t.x;
}
pub fn get_6(mut t: Local_7) -> i32 {
    return (t.x as i32);
}
pub fn twice_8(mut t: Local_3) -> i32 {
    return (t.x * 2);
}
pub fn wrap_9(mut v: i32) -> i32 {
    let mut l: Local_5 = Local_5 { x: v };
    return ({ get_4(l) });
}
pub fn wrap_10(mut v: i64) -> i32 {
    let mut l: Local_7 = Local_7 { x: v };
    return ({ get_6(l) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_5 {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Local_7 {
    #[offset(0)]
    pub x: i64,
}
pub fn other_11() -> i32 {
    let mut l: Local_1 = Local_1 { x: 3_i64, y: 4_i64 };
    return (({ get_0((l).clone()) }) + (l.y as i32));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Local_1 {
    #[offset(0)]
    pub x: i64,
    #[offset(8)]
    pub y: i64,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut l: Local_3 = Local_3 { x: 7 };
    assert!((({ get_2((l).clone(),) }) == 7));
    assert!((({ twice_8((l).clone(),) }) == 14));
    assert!((({ other_11() }) == 7));
    assert!((({ wrap_9(5,) }) == 5));
    assert!((({ wrap_10(6_i64,) }) == 6));
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_3 {
    #[offset(0)]
    pub x: i32,
}
pub fn __cpp2rust_init_globals() {}
