extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_4(t: Local_0) -> i32 {
    let t: Value<Local_0> = Rc::new(RefCell::new(t));
    return ({ (*t.borrow()).x } as i32);
}
pub fn get_5(t: Local_1) -> i32 {
    let t: Value<Local_1> = Rc::new(RefCell::new(t));
    return { (*t.borrow()).x };
}
pub fn get_6(t: Local_2) -> i32 {
    let t: Value<Local_2> = Rc::new(RefCell::new(t));
    return { (*t.borrow()).x };
}
pub fn get_7(t: Local_3) -> i32 {
    let t: Value<Local_3> = Rc::new(RefCell::new(t));
    return ({ (*t.borrow()).x } as i32);
}
pub fn twice_8(t: Local_1) -> i32 {
    let t: Value<Local_1> = Rc::new(RefCell::new(t));
    return ({ (*t.borrow()).x } * 2);
}
pub fn wrap_9(v: i32) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let l: Value<Local_2> = Rc::new(RefCell::new(Local_2 { x: (*v.borrow()) }));
    return ({ get_6((*l.borrow()).clone()) });
}
pub fn wrap_10(v: i64) -> i32 {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let l: Value<Local_3> = Rc::new(RefCell::new(Local_3 { x: (*v.borrow()) }));
    return ({ get_7((*l.borrow()).clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_2 {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Local_3 {
    #[offset(0)]
    pub x: i64,
}
pub fn other_11() -> i32 {
    let l: Value<Local_0> = Rc::new(RefCell::new(Local_0 { x: 3_i64, y: 4_i64 }));
    return (({ get_4((*l.borrow()).clone()) }) + ({ (*l.borrow()).y } as i32));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Local_0 {
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
    let l: Value<Local_1> = Rc::new(RefCell::new(Local_1 { x: 7 }));
    assert!((({ get_5((*l.borrow()).clone(),) }) == 7));
    assert!((({ twice_8((*l.borrow()).clone(),) }) == 14));
    assert!((({ other_11() }) == 7));
    assert!((({ wrap_9(5,) }) == 5));
    assert!((({ wrap_10(6_i64,) }) == 6));
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_1 {
    #[offset(0)]
    pub x: i32,
}
pub fn __cpp2rust_init_globals() {}
