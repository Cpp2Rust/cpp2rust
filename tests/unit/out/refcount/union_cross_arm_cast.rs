extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct shape_a {
    pub code: Value<u16>,
    pub pad: Value<Box<[u8]>>,
}
impl Default for shape_a {
    fn default() -> Self {
        shape_a {
            code: <Value<u16>>::default(),
            pad: Rc::new(RefCell::new(
                (0..14).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for shape_a {}
#[derive()]
pub struct shape_b {
    pub code: Value<u16>,
    pub lo: Value<u16>,
    pub mid: Value<u32>,
    pub fill: Value<Box<[u8]>>,
    pub tail: Value<u32>,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: <Value<u16>>::default(),
            lo: <Value<u16>>::default(),
            mid: <Value<u32>>::default(),
            fill: Rc::new(RefCell::new(
                (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
            tail: <Value<u32>>::default(),
        }
    }
}
impl ByteRepr for shape_b {}
#[derive()]
pub union Container_anon_22_3 {
    pub a: Value<shape_a>,
    pub b: Value<shape_b>,
    pub raw_: Value<Box<[u8]>>,
}
impl Default for Container_anon_22_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Container_anon_22_3 {}
#[derive(Default)]
pub struct Container {
    pub len: Value<u32>,
    pub u: Value<Container_anon_22_3>,
}
impl ByteRepr for Container {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Container> = <Value<Container>>::default();
    {
        ((c.as_pointer()) as Ptr<Container>).to_any().memset(
            (0) as u8,
            ::std::mem::size_of::<Container>() as u64 as usize,
        );
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    (*(*(*(*c.borrow()).u.borrow()).a.borrow()).code.borrow_mut()) = 10_u16;
    (*(*c.borrow()).len.borrow_mut()) = (::std::mem::size_of::<shape_b>() as u64 as u32);
    (*(*((((*(*c.borrow()).u.borrow()).a.as_pointer())
        .to_strong()
        .as_pointer() as AnyPtr)
        .cast::<shape_b>()
        .expect("ub:wrong type"))
    .upgrade()
    .deref())
    .tail
    .borrow_mut()) = 3735928559_u32;
    assert!(((*(*(*(*c.borrow()).u.borrow()).b.borrow()).tail.borrow()) == 3735928559_u32));
    assert!((((*(*(*(*c.borrow()).u.borrow()).b.borrow()).code.borrow()) as i32) == 10));
    (*(*(*(*c.borrow()).u.borrow()).b.borrow()).lo.borrow_mut()) = 8080_u16;
    assert!(
        ((((((*(*c.borrow()).u.borrow()).raw_.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((2) as isize)
            .read()) as i32)
            == 144)
    );
    assert!(
        ((((((*(*c.borrow()).u.borrow()).raw_.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((3) as isize)
            .read()) as i32)
            == 31)
    );
    return 0;
}
