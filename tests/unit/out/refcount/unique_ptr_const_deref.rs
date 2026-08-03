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
    pub val: Option<Value<i32>>,
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.val.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: <Option<Value<i32>>>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn read_val_0(h: Ptr<Holder>) -> i32 {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    return (*(*h.borrow())
        .with(|__v| (*__v).val.clone())
        .as_ref()
        .unwrap()
        .borrow());
}
pub fn write_val_1(h: Ptr<Holder>, v: i32) {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    (*(*h.borrow())
        .with(|__v| (*__v).val.clone())
        .as_ref()
        .unwrap()
        .borrow_mut()) = (*v.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (*h.borrow_mut()).val = Some(Rc::new(RefCell::new(10)));
    ({ write_val_1((h.as_pointer()), 42) });
    return ({ read_val_0((h.as_pointer())) });
}
