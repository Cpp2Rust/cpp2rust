extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct StructWithCtor {
    x1_: Value<i32>,
    x2_: Value<i32>,
}
impl StructWithCtor {
    pub fn StructWithCtor(x1: i32, x2: i32) -> Self {
        let x1: Value<i32> = Rc::new(RefCell::new(x1));
        let x2: Value<i32> = Rc::new(RefCell::new(x2));
        let mut this = Self {
            x1_: Rc::new(RefCell::new((*x1.borrow()))),
            x2_: Rc::new(RefCell::new((*x2.borrow()))),
        };
        (*this.x1_.borrow_mut()).prefix_inc();
        (*this.x2_.borrow_mut()).prefix_dec();
        this
    }
    pub fn x1(&self) -> Ptr<i32> {
        return self.x1_.as_pointer();
    }
    pub fn x2(&self) -> Ptr<i32> {
        return self.x2_.as_pointer();
    }
}
impl Clone for StructWithCtor {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1_: Rc::new(RefCell::new((*self.x1_.borrow()))),
            x2_: Rc::new(RefCell::new((*self.x2_.borrow()))),
        };
        this
    }
}
impl ByteRepr for StructWithCtor {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x1_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.x2_.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            x2_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn foo_0(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let struct_with_ctor: Value<StructWithCtor> =
        Rc::new(RefCell::new(StructWithCtor::StructWithCtor(1, 2)));
    let x: Value<i32> = Rc::new(RefCell::new(3));
    return (((((({ foo_0(x.as_pointer()) }).read()) == 3)
        && ((({ (*struct_with_ctor.borrow()).x1() }).read()) == 2))
        && ((({ (*struct_with_ctor.borrow()).x2() }).read()) == 1)) as i32);
}
