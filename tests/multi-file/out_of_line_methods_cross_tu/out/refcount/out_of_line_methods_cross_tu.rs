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
    fn scale(&self, x: i32) -> i32;
}
#[derive(Default)]
pub struct D {
    pub f: Value<i32>,
}
impl D {}
impl Base for D {}
impl Clone for D {
    fn clone(&self) -> Self {
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            f: Rc::new(RefCell::new((*self.f.borrow()))),
        }));
        let this: Ptr<D> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for D {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::S({ 1 })));
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 1));
    ({ SImpl::set(&s.as_pointer(), 4) });
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 4));
    assert!((({ SImpl::add(&s.as_pointer(), 2,) }) == 6));
    let d: Value<D> = Rc::new(RefCell::new(D::D({ 3 })));
    let b: Value<PtrDyn<dyn Base>> =
        Rc::new(RefCell::new((d.as_pointer()).to_dyn::<dyn Base>(|w| w)));
    assert!((({ (*(*b.borrow()).upgrade().deref()).scale(5,) }) == 15));
    return 0;
}
impl S {
    pub fn S(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*x.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl D {
    pub fn D(f: i32) -> Self {
        let f: Value<i32> = Rc::new(RefCell::new(f));
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            f: Rc::new(RefCell::new((*f.borrow()))),
        }));
        let this: Ptr<D> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl S {}
impl D {}
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
