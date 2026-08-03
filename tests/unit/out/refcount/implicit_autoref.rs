extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Holder {
    pub v: Vec<i32>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: (self.v).clone(),
        };
        this
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <Vec<i32>>::from_bytes(&buf[0..24]),
        }
    }
}
pub fn write_through_0(p: Ptr<i32>) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    (*p.borrow()).write(42);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(10);
    (*v.borrow_mut()).push(20);
    let p: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new((v.as_pointer())));
    let a: Value<i32> = Rc::new(RefCell::new(
        ((((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
            .offset(0_usize)
            .read()),
    ));
    (((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
        .offset(1_usize)
        .write(30);
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (*h.borrow_mut()).v.push(40);
    (*h.borrow_mut()).v.push(50);
    let hp: Value<Ptr<Holder>> = Rc::new(RefCell::new((h.as_pointer())));
    let b: Value<i32> = Rc::new(RefCell::new(
        (((*hp.borrow()).field_ptr(
            0,
            |__v: &Holder| &__v.v[..],
            |__v: &mut Holder| &mut __v.v[..],
        ) as Ptr<i32>)
            .offset(0_usize)
            .read()),
    ));
    ((*hp.borrow()).field_ptr(
        0,
        |__v: &Holder| &__v.v[..],
        |__v: &mut Holder| &mut __v.v[..],
    ) as Ptr<i32>)
        .offset(1_usize)
        .write(60);
    assert!(((*a.borrow()) == 10));
    assert!(
        (((((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
            .offset(1_usize)
            .read())
            == 30)
    );
    assert!(((*b.borrow()) == 40));
    assert!(
        ((((*hp.borrow()).field_ptr(
            0,
            |__v: &Holder| &__v.v[..],
            |__v: &mut Holder| &mut __v.v[..]
        ) as Ptr<i32>)
            .offset(1_usize)
            .read())
            == 60)
    );
    ({
        write_through_0(
            (((*p.borrow()).to_strong().as_pointer() as Ptr<i32>).offset(0_usize as isize)),
        )
    });
    assert!(
        (((((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
            .offset(0_usize)
            .read())
            == 42)
    );
    return 0;
}
