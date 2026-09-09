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
    pub buf: Value<Ptr<i32>>,
    pub n: Value<usize>,
}
impl Base {
    pub fn Base(b: Ptr<i32>, n: usize) -> Self {
        let b: Value<Ptr<i32>> = Rc::new(RefCell::new(b));
        let n: Value<usize> = Rc::new(RefCell::new(n));
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            buf: Rc::new(RefCell::new((*b.borrow()).clone())),
            n: Rc::new(RefCell::new((*n.borrow()))),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Base {
    fn clone(&self) -> Self {
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            buf: Rc::new(RefCell::new((*self.buf.borrow()).clone())),
            n: Rc::new(RefCell::new((*self.n.borrow()))),
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
        (*self.buf.borrow()).to_bytes(&mut buf[0..8]);
        (*self.n.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            buf: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
            n: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct Derived {
    pub base_Base: Value<Base>,
}
impl Derived {
    pub fn Derived(_: Ptr<i32>, _: usize) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new()),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new((*self.base_Base.borrow()).clone())),
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
        (*self.base_Base.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let d: Value<Derived> = Rc::new(RefCell::new(Derived::Derived(
        { (arr.as_pointer() as Ptr<i32>) },
        { 3_usize },
    )));
    assert!(((*(*(*d.borrow()).base_Base.borrow()).n.borrow()) == 3_usize));
    assert!(
        (((*(*(*d.borrow()).base_Base.borrow()).buf.borrow())
            .offset((1) as isize)
            .read())
            == 2)
    );
    assert!(((({ DerivedImpl::begin(&d.as_pointer(),) }).read()) == 1));
    assert!(
        ((({ DerivedImpl::end(&d.as_pointer(),) }) - ({ DerivedImpl::begin(&d.as_pointer(),) }))
            as i64
            == 3_i64)
    );
    return 0;
}
pub trait DerivedImpl {
    fn begin(&self) -> Ptr<i32>;
    fn end(&self) -> Ptr<i32>;
}
impl DerivedImpl for Ptr<Derived> {
    fn begin(&self) -> Ptr<i32> {
        return (*(*(*(*self).upgrade().deref()).base_Base.borrow())
            .buf
            .borrow())
        .clone();
    }
    fn end(&self) -> Ptr<i32> {
        return (*(*(*(*self).upgrade().deref()).base_Base.borrow())
            .buf
            .borrow())
        .offset(
            (*(*(*(*self).upgrade().deref()).base_Base.borrow())
                .n
                .borrow()) as isize,
        );
    }
}
