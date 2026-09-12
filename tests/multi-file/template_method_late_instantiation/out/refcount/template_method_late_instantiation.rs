extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct S_int_ {
    pub x: Value<i32>,
}
impl Clone for S_int_ {
    fn clone(&self) -> Self {
        let __this: Value<S_int_> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<S_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S_int_ {
    fn default() -> Self {
        S_int_ {
            x: Rc::new(RefCell::new(0)),
        }
    }
}
impl ByteRepr for S_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<S_int_> = Rc::new(RefCell::new(<S_int_>::default()));
    ({ S_int_Impl::set(&p.as_pointer(), 3) });
    assert!((({ f_0((p.as_pointer()),) }) == 3));
    return 0;
}
pub fn f_0(p: Ptr<S_int_>) -> i32 {
    let p: Value<Ptr<S_int_>> = Rc::new(RefCell::new(p));
    return ({ S_int_Impl::get(&(*p.borrow())) });
}
pub trait S_int_Impl {
    fn set(&self, v: i32);
    fn get(&self) -> i32;
}
impl S_int_Impl for Ptr<S_int_> {
    fn set(&self, v: i32) {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        (*(*(*self).upgrade().deref()).x.borrow_mut()) = (*v.borrow());
    }
    fn get(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).x.borrow());
    }
}
