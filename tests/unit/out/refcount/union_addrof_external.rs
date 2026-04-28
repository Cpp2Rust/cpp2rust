extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive()]
pub struct record {
    pub code: Value<u16>,
    pub lo: Value<u16>,
    pub hi: Value<u32>,
    pub pad: Value<Box<[u8]>>,
}
impl ByteRepr for record {}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Container_anon_15_3 {
    pub h: Value<record>,
    pub raw_: Value<Box<[u8]>>,
}
impl Default for Container_anon_15_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Container {
    pub view: Value<Container_anon_15_3>,
}
impl ByteRepr for Container {}
pub fn fill_0(out: AnyPtr, cap: u64) {
    let out: Value<AnyPtr> = Rc::new(RefCell::new(out));
    let cap: Value<u64> = Rc::new(RefCell::new(cap));
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
    (*src.borrow_mut())[(1) as usize] = 0_u8;
    (*src.borrow_mut())[(2) as usize] = 0_u8;
    (*src.borrow_mut())[(3) as usize] = 80_u8;
    (*src.borrow_mut())[(4) as usize] = 127_u8;
    (*src.borrow_mut())[(5) as usize] = 0_u8;
    (*src.borrow_mut())[(6) as usize] = 0_u8;
    (*src.borrow_mut())[(7) as usize] = 1_u8;
    let n: Value<u64> = Rc::new(RefCell::new(
        if ((::std::mem::size_of::<[u8; 16]>() as u64) < (*cap.borrow())) {
            ::std::mem::size_of::<[u8; 16]>() as u64
        } else {
            (*cap.borrow())
        },
    ));
    {
        (*out.borrow()).memcpy(
            &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            (*n.borrow()) as usize,
        );
        (*out.borrow()).clone()
    };
}
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
    ({
        let _out: AnyPtr = (((*c.borrow()).view.as_pointer()).to_strong().as_pointer() as AnyPtr);
        let _cap: u64 = ::std::mem::size_of::<Container_anon_15_3>() as u64;
        fill_0(_out, _cap)
    });
    assert!((((*(*(*(*c.borrow()).view.borrow()).h.borrow()).code.borrow()) as i32) == (2)));
    assert!(
        ((((((*(*(*c.borrow()).view.borrow()).h.borrow()).lo.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == (0))
    );
    assert!(
        ((((((*(*(*c.borrow()).view.borrow()).h.borrow()).lo.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((1) as isize)
            .read()) as i32)
            == (80))
    );
    assert!((((*(*(*c.borrow()).view.borrow()).raw_.borrow())[(0) as usize] as i32) == (2)));
    assert!(
        ((((*(*(*c.borrow()).view.borrow()).raw_.borrow())[(3) as usize] as u8) as i32) == (80))
    );
    return 0;
}
