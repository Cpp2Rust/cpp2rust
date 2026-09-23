extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub v: Value<i32>,
}
impl S {}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
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
pub trait Base {
    fn apply(&self, x: i32) -> i32;
}
#[derive(Default)]
pub struct Derived {
    pub factor: Value<i32>,
}
impl Derived {}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            factor: Rc::new(RefCell::new((*self.factor.borrow()))),
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
        (*self.factor.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            factor: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 1));
    ({ SImpl::set(&s.as_pointer(), 4) });
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 4));
    assert!((({ SImpl::add(&s.as_pointer(), 2,) }) == 6));
    let derived: Value<Derived> = Rc::new(RefCell::new(Derived::new({ 3 })));
    let base: Value<PtrDyn<dyn Base>> = Rc::new(RefCell::new(
        (derived.as_pointer()).to_dyn::<dyn Base>(|w| w),
    ));
    assert!((({ (*(*base.borrow()).upgrade().deref()).apply(5,) }) == 15));
    return 0;
}
impl S {
    pub fn new(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*x.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Derived {
    pub fn new(factor: i32) -> Self {
        let factor: Value<i32> = Rc::new(RefCell::new(factor));
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl S {}
impl Derived {}
impl Base for Derived {
    fn apply(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*self.factor.borrow()) * (*x.borrow()));
    }
}
pub trait SImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn get(&self) -> i32;
    fn set(&self, x: i32) {
        unimplemented!()
    }
    fn add(&self, x: i32) -> i32 {
        unimplemented!()
    }
}
impl SImpl for Ptr<S> {
    fn get(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).v.borrow());
    }
    fn destructor(&self) {}
    fn set(&self, x: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = (*x.borrow());
    }
    fn add(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) += (*x.borrow());
        return (*(*(*self).upgrade().deref()).v.borrow());
    }
}
pub fn __cpp2rust_init_globals() {}
