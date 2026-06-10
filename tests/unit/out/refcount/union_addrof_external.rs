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
    pub lo: Value<u16>,
    pub hi: Value<u32>,
    pub pad: Value<Box<[u8]>>,
}
impl Default for record {
    fn default() -> Self {
        record {
            code: <Value<u16>>::default(),
            lo: <Value<u16>>::default(),
            hi: <Value<u32>>::default(),
            pad: Rc::new(RefCell::new(
                (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for record {}
#[derive()]
pub union anon_0 {
    pub h: Value<record>,
    pub raw_: Value<Box<[u8]>>,
}
impl ByteRepr for anon_0 {}
#[derive(Default)]
pub struct Container {
    pub view: Value<anon_0>,
}
impl ByteRepr for Container {}
pub fn fill_1(out: AnyPtr, cap: usize) {
    let out: Value<AnyPtr> = Rc::new(RefCell::new(out));
    let cap: Value<usize> = Rc::new(RefCell::new(cap));
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
    let n: Value<usize> = Rc::new(RefCell::new(
        if (((::std::mem::size_of::<[u8; 16]>() < (*cap.borrow())) as i32) != 0) {
            ::std::mem::size_of::<[u8; 16]>()
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
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<Container>() as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    ({
        let _out: AnyPtr = ((*c.borrow()).view.as_pointer()).to_any();
        let _cap: usize = ::std::mem::size_of::<anon_0>();
        fill_1(_out, _cap)
    });
    assert!(
        (((((*(*(*(*c.borrow()).view.borrow()).h.borrow()).code.borrow()) as i32) == 2) as i32)
            != 0)
    );
    assert!(
        ((((((((*(*(*c.borrow()).view.borrow()).h.borrow()).lo.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((((((((*(*(*c.borrow()).view.borrow()).h.borrow()).lo.as_pointer())
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((1) as isize)
            .read()) as i32)
            == 80) as i32)
            != 0)
    );
    assert!(
        (((((*(*(*c.borrow()).view.borrow()).raw_.borrow())[(0) as usize] as i32) == 2) as i32)
            != 0)
    );
    assert!(
        ((((((*(*(*c.borrow()).view.borrow()).raw_.borrow())[(3) as usize] as u8) as i32) == 80)
            as i32)
            != 0)
    );
    return 0;
}
