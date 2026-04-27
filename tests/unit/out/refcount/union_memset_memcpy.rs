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
    pub hi: Value<u32>,
    pub fill: Value<Box<[u8]>>,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: <Value<u16>>::default(),
            lo: <Value<u16>>::default(),
            hi: <Value<u32>>::default(),
            fill: Rc::new(RefCell::new(
                (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for shape_b {}
#[derive()]
pub union Container_anon_0 {
    pub a: Value<shape_a>,
    pub b: Value<shape_b>,
    pub raw_: Value<Box<[u8]>>,
}
impl Default for Container_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Container_anon_0 {}
#[derive(Default)]
pub struct Container {
    pub view: Value<Container_anon_0>,
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
    assert!((((*(*(*(*c.borrow()).view.borrow()).a.borrow()).code.borrow()) as i32) == 0));
    assert!((((*(*(*(*c.borrow()).view.borrow()).b.borrow()).lo.borrow()) as i32) == 0));
    assert!((((*(*(*c.borrow()).view.borrow()).raw_.borrow())[(0) as usize] as i32) == 0));
    assert!((((*(*(*c.borrow()).view.borrow()).raw_.borrow())[(255) as usize] as i32) == 0));
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    (*src.borrow_mut())[(0) as usize] = 2_u8;
    (*src.borrow_mut())[(2) as usize] = 80_u8;
    (*src.borrow_mut())[(3) as usize] = 0_u8;
    (*src.borrow_mut())[(4) as usize] = 127_u8;
    (*src.borrow_mut())[(5) as usize] = 0_u8;
    (*src.borrow_mut())[(6) as usize] = 0_u8;
    (*src.borrow_mut())[(7) as usize] = 1_u8;
    let len: Value<u64> = Rc::new(RefCell::new(16_u64));
    assert!(((*len.borrow()) <= ::std::mem::size_of::<[u8; 256]>() as u64));
    {
        (((*(*c.borrow()).view.borrow()).raw_.as_pointer()) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                (*len.borrow()) as usize,
            );
        (((*(*c.borrow()).view.borrow()).raw_.as_pointer()) as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!((((*(*(*(*c.borrow()).view.borrow()).b.borrow()).code.borrow()) as i32) == 2));
    assert!(
        ((((((*(*(*c.borrow()).view.borrow()).b.borrow()).lo.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == 80)
    );
    {
        ((c.as_pointer()) as Ptr<Container>).to_any().memset(
            (0) as u8,
            ::std::mem::size_of::<Container>() as u64 as usize,
        );
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    assert!((((*(*(*(*c.borrow()).view.borrow()).b.borrow()).code.borrow()) as i32) == 0));
    return 0;
}
