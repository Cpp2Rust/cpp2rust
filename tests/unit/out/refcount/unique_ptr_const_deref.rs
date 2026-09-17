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
    pub val: Value<Option<Value<i32>>>,
}
impl Holder {
    pub fn Holder_pmutHolder(_a0: Ptr<Holder>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            val: Rc::new(RefCell::new(
                (*(*_a0.upgrade().deref()).val.borrow_mut()).take(),
            )),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.val.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: Rc::new(RefCell::new(<Option<Value<i32>>>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn read_val_0(h: Ptr<Holder>) -> i32 {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    return (*(*(*(*h.borrow()).upgrade().deref()).val.borrow())
        .as_ref()
        .unwrap()
        .borrow());
}
pub fn write_val_1(h: Ptr<Holder>, v: i32) {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    (*(*(*(*h.borrow()).upgrade().deref()).val.borrow_mut())
        .as_ref()
        .unwrap()
        .borrow_mut()) = (*v.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    ((*h.borrow()).val.as_pointer() as Ptr<Option<Value<i32>>>)
        .write(Some(Rc::new(RefCell::new(10))).take());
    ({ write_val_1((h.as_pointer()), 42) });
    assert!((({ read_val_0((h.as_pointer()),) }) == 42));
    return 0;
}
pub trait HolderImpl {
    fn operator_assign_pmutHolder(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn operator_assign_pmutHolder(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        ((*(*self).upgrade().deref()).val.as_pointer() as Ptr<Option<Value<i32>>>)
            .write((*(*_a0.upgrade().deref()).val.borrow_mut()).take());
        return (*self).clone();
    }
}
