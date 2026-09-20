extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct S {
    pub a: Value<i32>,
    pub b: Value<bool>,
}
impl S {
    pub fn S() -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new(11)),
            b: Rc::new(RefCell::new(true)),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::S() }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<bool>::from_bytes(&buf[4..5]))),
        }
    }
}
#[derive(Default)]
pub struct Declared {
    pub v: Value<i32>,
}
impl Declared {}
impl Clone for Declared {
    fn clone(&self) -> Self {
        let __this: Value<Declared> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Declared> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Declared {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<Declared>> = Rc::new(RefCell::new(Ptr::<Declared>::null()));
    assert!((*d.borrow()).is_null());
    let s: Value<S> = Rc::new(RefCell::new(S::S()));
    assert!(((*(*s.borrow()).a.borrow()) == 11));
    assert!((((*(*s.borrow()).b.borrow()) as i32) == (true as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
