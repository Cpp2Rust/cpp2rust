extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Base {
    pub buf: Value<Box<[u8]>>,
}
impl Clone for Base {
    fn clone(&self) -> Self {
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            buf: Rc::new(RefCell::new((*self.buf.borrow()).clone())),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Base {
    fn default() -> Self {
        Base {
            buf: Rc::new(RefCell::new(
                (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for Base {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.buf.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            buf: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct Derived {
    pub base_Base: Value<Base>,
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
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Base.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Derived> = Rc::new(RefCell::new(<Derived>::default()));
    assert!(({ DerivedImpl::run(&d.as_pointer(),) }));
    ({ BaseImpl::fill(&(*d.borrow()).base_Base.as_pointer(), ('y' as u8), 1) });
    assert!(
        (((*(*(*d.borrow()).base_Base.borrow()).buf.borrow())[(0) as usize] as i32)
            == (('y' as u8) as i32))
    );
    assert!(
        (((*(*(*d.borrow()).base_Base.borrow()).buf.borrow())[(1) as usize] as i32)
            == (('x' as u8) as i32))
    );
    return 0;
}
pub trait BaseImpl {
    fn fill(&self, c: u8, n: i32);
}
impl BaseImpl for Ptr<Base> {
    fn fill(&self, c: u8, n: i32) {
        let c: Value<u8> = Rc::new(RefCell::new(c));
        let n: Value<i32> = Rc::new(RefCell::new(n));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*n.borrow())) {
            (*(*(*self).upgrade().deref()).buf.borrow_mut())[(*i.borrow()) as usize] =
                (*c.borrow());
            (*i.borrow_mut()).postfix_inc();
        }
    }
}
pub trait DerivedImpl {
    fn run(&self) -> bool;
}
impl DerivedImpl for Ptr<Derived> {
    fn run(&self) -> bool {
        ({
            BaseImpl::fill(
                &(*(*self).upgrade().deref()).base_Base.as_pointer(),
                ('x' as u8),
                3,
            )
        });
        return (((*(*(*(*self).upgrade().deref()).base_Base.borrow())
            .buf
            .borrow())[(2) as usize] as i32)
            == (('x' as u8) as i32));
    }
}
