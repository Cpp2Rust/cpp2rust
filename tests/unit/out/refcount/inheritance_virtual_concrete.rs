extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Base {
    pub v: Value<i32>,
}
impl Clone for Base {
    fn clone(&self) -> Self {
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Base {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct Derived {
    pub __base: Value<Base>,
    pub w: Value<i32>,
}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            __base: Rc::new(RefCell::new((self as Base).clone())),
            w: Rc::new(RefCell::new((*self.w.borrow()))),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Derived {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.__base.borrow()).to_bytes(&mut buf[0..16]);
        (*self.w.borrow()).to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            __base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..16]))),
            w: Rc::new(RefCell::new(<i32>::from_bytes(&buf[12..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let b: Value<Base> = Rc::new(RefCell::new(<Base>::default()));
    let d: Value<Derived> = Rc::new(RefCell::new(<Derived>::default()));
    let p: Value<Ptr<Base>> = Rc::new(RefCell::new((d.as_pointer())));
    assert!((({ (*b.borrow()).get() }) == 1));
    assert!((({ (*(*p.borrow()).upgrade().deref()).get() }) == 3));
    assert!(((*(*(*p.borrow()).upgrade().deref()).v.borrow()) == 1));
    return 0;
}
