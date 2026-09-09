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
    pub a_: Value<i16>,
    pub b_: Value<i8>,
}
impl Base {
    pub fn Base(a: i16, b: i8) -> Self {
        let a: Value<i16> = Rc::new(RefCell::new(a));
        let b: Value<i8> = Rc::new(RefCell::new(b));
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            a_: Rc::new(RefCell::new((*a.borrow()))),
            b_: Rc::new(RefCell::new((*b.borrow()))),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Base {
    fn clone(&self) -> Self {
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            a_: Rc::new(RefCell::new((*self.a_.borrow()))),
            b_: Rc::new(RefCell::new((*self.b_.borrow()))),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Base {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a_.borrow()).to_bytes(&mut buf[0..2]);
        (*self.b_.borrow()).to_bytes(&mut buf[2..3]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a_: Rc::new(RefCell::new(<i16>::from_bytes(&buf[0..2]))),
            b_: Rc::new(RefCell::new(<i8>::from_bytes(&buf[2..3]))),
        }
    }
}
#[derive(Default)]
pub struct Derived {
    pub base_Base: Value<Base>,
    pub c_: Value<i8>,
}
impl Derived {
    pub fn Derived(a: i16, b: i8, c: i8) -> Self {
        let a: Value<i16> = Rc::new(RefCell::new(a));
        let b: Value<i8> = Rc::new(RefCell::new(b));
        let c: Value<i8> = Rc::new(RefCell::new(c));
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new(Base::Base({ (*a.borrow()) }, {
                (*b.borrow())
            }))),
            c_: Rc::new(RefCell::new((*c.borrow()))),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new((self as Base).clone())),
            c_: Rc::new(RefCell::new((*self.c_.borrow()))),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Derived {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Base.borrow()).to_bytes(&mut buf[0..4]);
        (*self.c_.borrow()).to_bytes(&mut buf[3..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..4]))),
            c_: Rc::new(RefCell::new(<i8>::from_bytes(&buf[3..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<Derived> = Rc::new(RefCell::new(Derived::Derived({ 1_i16 }, { 2_i8 }, {
        3_i8
    })));
    let dst: Value<Derived> = Rc::new(RefCell::new(Derived::Derived({ 4_i16 }, { 5_i8 }, {
        6_i8
    })));
    let s: Value<Ptr<Base>> = Rc::new(RefCell::new((src.as_pointer()).reinterpret_cast::<Base>()));
    let t: Value<Ptr<Base>> = Rc::new(RefCell::new((dst.as_pointer()).reinterpret_cast::<Base>()));
    let __rhs = (*(*s.borrow()).upgrade().deref()).clone();
    (*t.borrow()).write(__rhs);
    assert!((((*((*dst.borrow()) as Base).a_.borrow()) as i32) == 1));
    assert!((((*((*dst.borrow()) as Base).b_.borrow()) as i32) == 2));
    assert!((((*(*dst.borrow()).c_.borrow()) as i32) == 6));
    return 0;
}
