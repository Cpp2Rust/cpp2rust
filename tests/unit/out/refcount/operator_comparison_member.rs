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
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(1)),
    }));
    let b: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(2)),
    }));
    let c: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(1)),
    }));
    assert!(({ SImpl::operator_eq(&a.as_pointer(), c.as_pointer(),) }));
    assert!(({ SImpl::operator_ne(&a.as_pointer(), b.as_pointer(),) }));
    assert!(({ SImpl::operator_lt_pconstS_const(&a.as_pointer(), b.as_pointer(),) }));
    assert!(({ SImpl::operator_gt(&b.as_pointer(), a.as_pointer(),) }));
    assert!(({ SImpl::operator_le(&a.as_pointer(), c.as_pointer(),) }));
    assert!(({ SImpl::operator_ge(&a.as_pointer(), c.as_pointer(),) }));
    assert!(!({ SImpl::operator_lt_pconstS_const(&b.as_pointer(), a.as_pointer(),) }));
    assert!(({ SImpl::operator_lt_i32_const(&a.as_pointer(), 5,) }));
    return 0;
}
pub trait SImpl {
    fn operator_eq(&self, o: Ptr<S>) -> bool;
    fn operator_ne(&self, o: Ptr<S>) -> bool;
    fn operator_lt_pconstS_const(&self, o: Ptr<S>) -> bool;
    fn operator_gt(&self, o: Ptr<S>) -> bool;
    fn operator_le(&self, o: Ptr<S>) -> bool;
    fn operator_ge(&self, o: Ptr<S>) -> bool;
    fn operator_lt_i32_const(&self, o: i32) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs == (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_ne(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs != (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_lt_pconstS_const(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs < (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_gt(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs > (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_le(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs <= (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_ge(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs >= (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_lt_i32_const(&self, o: i32) -> bool {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return ((*(*(*self).upgrade().deref()).v.borrow()) < (*o.borrow()));
    }
}
