extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct record {
    pub code: Value<u16>,
    pub pad: Value<Box<[u8]>>,
}
impl Default for record {
    fn default() -> Self {
        record {
            code: <Value<u16>>::default(),
            pad: Rc::new(RefCell::new(
                (0..14).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for record {}
#[derive()]
pub union inner_anon_0 {
    pub h: Value<record>,
    pub raw_: Value<Box<[u8]>>,
}
impl Default for inner_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for inner_anon_0 {}
#[derive(Default)]
pub struct inner {
    pub view: Value<inner_anon_0>,
}
impl ByteRepr for inner {}
#[derive(Default)]
pub union Outer_anon_0 {
    pub h: Value<record>,
    pub nested: Value<inner>,
}
impl Default for Outer_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Outer_anon_0 {}
#[derive(Default)]
pub struct Outer {
    pub kind: Value<i32>,
    pub level: Value<i32>,
    pub variant: Value<i32>,
    pub len: Value<u32>,
    pub body: Value<Outer_anon_0>,
}
impl ByteRepr for Outer {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ex: Value<Outer> = <Value<Outer>>::default();
    {
        ((ex.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<Outer>() as u64 as usize);
        ((ex.as_pointer()) as Ptr<Outer>).to_any().clone()
    };
    (*(*ex.borrow()).kind.borrow_mut()) = 2;
    (*(*ex.borrow()).level.borrow_mut()) = 1;
    (*(*ex.borrow()).variant.borrow_mut()) = 6;
    (*(*ex.borrow()).len.borrow_mut()) = (::std::mem::size_of::<record>() as u64 as u32);
    (*(*(*(*ex.borrow()).body.borrow()).h.borrow())
        .code
        .borrow_mut()) = 2_u16;
    (*(*(*(*ex.borrow()).body.borrow()).h.borrow())
        .pad
        .borrow_mut())[(0) as usize] = (('X' as i32) as u8);
    assert!((((*(*(*(*ex.borrow()).body.borrow()).h.borrow()).code.borrow()) as i32) == 2));
    assert!(
        (((*(*(*(*ex.borrow()).body.borrow()).h.borrow()).pad.borrow())[(0) as usize] as i32)
            == ('X' as i32))
    );
    assert!(
        (((*(*(*(*(*(*ex.borrow()).body.borrow()).nested.borrow())
            .view
            .borrow())
        .h
        .borrow())
        .code
        .borrow()) as i32)
            == 2)
    );
    return 0;
}
