extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl ByteRepr for Inner {}
pub fn sum_inner_0(i: Ptr<Inner>) -> i32 {
    let i: Value<Ptr<Inner>> = Rc::new(RefCell::new(i));
    return {
        let _lhs = (*(*(*i.borrow()).upgrade().deref()).a.borrow());
        _lhs + (*(*(*i.borrow()).upgrade().deref()).b.borrow())
    };
}
#[derive()]
pub union Outer_anon_0 {
    pub inner: Value<Inner>,
    pub raw_: Value<Box<[u8]>>,
}
impl Default for Outer_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Outer_anon_0 {}
#[derive(Default)]
pub struct Outer {
    pub u: Value<Outer_anon_0>,
}
impl ByteRepr for Outer {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let standalone: Value<Inner> = <Value<Inner>>::default();
    (*(*standalone.borrow()).a.borrow_mut()) = 3;
    (*(*standalone.borrow()).b.borrow_mut()) = 4;
    assert!(
        (({
            let _i: Ptr<Inner> = (standalone.as_pointer());
            sum_inner_0(_i)
        }) == 7)
    );
    let outer: Value<Outer> = <Value<Outer>>::default();
    {
        ((outer.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<Outer>() as u64 as usize);
        ((outer.as_pointer()) as Ptr<Outer>).to_any().clone()
    };
    (*(*(*(*outer.borrow()).u.borrow()).inner.borrow())
        .a
        .borrow_mut()) = 3;
    (*(*(*(*outer.borrow()).u.borrow()).inner.borrow())
        .b
        .borrow_mut()) = 4;
    assert!(
        (({
            let _i: Ptr<Inner> = ((*(*outer.borrow()).u.borrow()).inner.as_pointer());
            sum_inner_0(_i)
        }) == 7)
    );
    assert!(((((*(*(*outer.borrow()).u.borrow()).raw_.borrow())[(0) as usize] as u8) as i32) == 3));
    assert!(((((*(*(*outer.borrow()).u.borrow()).raw_.borrow())[(4) as usize] as u8) as i32) == 4));
    return 0;
}
