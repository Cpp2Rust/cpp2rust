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
    pub fn new(x1: i32, x2: i32) -> Self {
        let x1: Value<i32> = Rc::new(RefCell::new(x1));
        let x2: Value<i32> = Rc::new(RefCell::new(x2));
        let __this: Value<StructWithCtor> = Rc::new(RefCell::new(Self {
            x1_: Rc::new(RefCell::new((*x1.borrow()))),
            x2_: Rc::new(RefCell::new((*x2.borrow()))),
        }));
        let this: Ptr<StructWithCtor> = __this.as_pointer();
        (*(*this.upgrade().deref()).x1_.borrow_mut()).prefix_inc();
        (*(*this.upgrade().deref()).x2_.borrow_mut()).prefix_dec();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for StructWithCtor {
    fn clone(&self) -> Self {
        let __this: Value<StructWithCtor> = Rc::new(RefCell::new(Self {
            x1_: Rc::new(RefCell::new((*self.x1_.borrow()))),
            x2_: Rc::new(RefCell::new((*self.x2_.borrow()))),
        }));
        let this: Ptr<StructWithCtor> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
#[derive(Default)]
pub struct Value_ {
    pub v: Value<i32>,
}
impl Value_ {
    pub fn new(u: i32) -> Self {
        let u: Value<i32> = Rc::new(RefCell::new(u));
        let __this: Value<Value_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*u.borrow()))),
        }));
        let this: Ptr<Value_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Value_ {
    fn clone(&self) -> Self {
        let __this: Value<Value_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Value_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Value_ {
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
#[derive()]
pub struct Ptr_ {
    pub v1: Value<Value_>,
    pub v2: Value<Value_>,
}
impl Ptr_ {
    pub fn new() -> Self {
        let __this: Value<Ptr_> = Rc::new(RefCell::new(Self {
            v1: Rc::new(RefCell::new(Value_::new({ 11 }))),
            v2: Rc::new(RefCell::new(Value_::new({ 22 }))),
        }));
        let this: Ptr<Ptr_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Ptr_ {
    fn clone(&self) -> Self {
        let __this: Value<Ptr_> = Rc::new(RefCell::new(Self {
            v1: Rc::new(RefCell::new((*self.v1.borrow()).clone())),
            v2: Rc::new(RefCell::new((*self.v2.borrow()).clone())),
        }));
        let this: Ptr<Ptr_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        { Ptr_::new() }
    }
}
impl ByteRepr for Ptr_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v1.borrow()).to_bytes(&mut buf[0..4]);
        (*self.v2.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v1: Rc::new(RefCell::new(<Value_>::from_bytes(&buf[0..4]))),
            v2: Rc::new(RefCell::new(<Value_>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let struct_with_ctor: Value<StructWithCtor> =
        Rc::new(RefCell::new(StructWithCtor::new({ 1 }, { 2 })));
    let x: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (((({ foo_0(x.as_pointer(),) }).read()) == 3)
            && ((({ StructWithCtorImpl::x1(&struct_with_ctor.as_pointer(),) }).read()) == 2))
            && ((({ StructWithCtorImpl::x2(&struct_with_ctor.as_pointer(),) }).read()) == 1)
    );
    let p: Value<Ptr_> = Rc::new(RefCell::new(Ptr_::new()));
    assert!(((*(*(*p.borrow()).v1.borrow()).v.borrow()) == 11));
    assert!(((*(*(*p.borrow()).v2.borrow()).v.borrow()) == 22));
    return 0;
}
pub trait StructWithCtorImpl {
    fn x1(&self) -> Ptr<i32>;
    fn x2(&self) -> Ptr<i32>;
}
impl StructWithCtorImpl for Ptr<StructWithCtor> {
    fn x1(&self) -> Ptr<i32> {
        return (*(*self).upgrade().deref()).x1_.as_pointer();
    }
    fn x2(&self) -> Ptr<i32> {
        return (*(*self).upgrade().deref()).x2_.as_pointer();
    }
}
pub fn __cpp2rust_init_globals() {}
