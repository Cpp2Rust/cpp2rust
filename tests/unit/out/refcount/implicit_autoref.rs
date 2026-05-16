extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Holder {
    pub v: Value<Vec<i32>>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Holder {}
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
            .offset(0_u64 as isize)
            .read()),
    ));
    (((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
        .offset(1_u64 as isize)
        .write(30);
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (*(*h.borrow()).v.borrow_mut()).push(40);
    (*(*h.borrow()).v.borrow_mut()).push(50);
    let hp: Value<Ptr<Holder>> = Rc::new(RefCell::new((h.as_pointer())));
    let b: Value<i32> = Rc::new(RefCell::new(
        (((*(*hp.borrow()).upgrade().deref()).v.as_pointer() as Ptr<i32>)
            .offset(0_u64 as isize)
            .read()),
    ));
    ((*(*hp.borrow()).upgrade().deref()).v.as_pointer() as Ptr<i32>)
        .offset(1_u64 as isize)
        .write(60);
    assert!(((*a.borrow()) == 10));
    assert!(
        (((((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
            .offset(1_u64 as isize)
            .read())
            == 30)
    );
    assert!(((*b.borrow()) == 40));
    assert!(
        ((((*(*hp.borrow()).upgrade().deref()).v.as_pointer() as Ptr<i32>)
            .offset(1_u64 as isize)
            .read())
            == 60)
    );
    ({
        let _p: Ptr<i32> =
            (((*p.borrow()).to_strong().as_pointer() as Ptr<i32>).offset(0_u64 as isize));
        write_through_0(_p)
    });
    assert!(
        (((((*p.borrow()).to_strong().as_pointer()) as Ptr<i32>)
            .offset(0_u64 as isize)
            .read())
            == 42)
    );
    return 0;
}
