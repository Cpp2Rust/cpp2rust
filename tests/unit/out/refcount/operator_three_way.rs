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
    assert!(
        ({ SImpl::operator_cmp(&a.as_pointer(), b.as_pointer(),) }) == std::cmp::Ordering::Less
    );
    assert!(
        ({ SImpl::operator_cmp(&b.as_pointer(), a.as_pointer(),) }) == std::cmp::Ordering::Greater
    );
    assert!(
        ({ SImpl::operator_cmp(&a.as_pointer(), b.as_pointer(),) }) != std::cmp::Ordering::Greater
    );
    assert!(
        ({ SImpl::operator_cmp(&b.as_pointer(), a.as_pointer(),) }) != std::cmp::Ordering::Less
    );
    assert!(!({ SImpl::operator_eq(&a.as_pointer(), b.as_pointer(),) }));
    assert!(
        ({ SImpl::operator_cmp(&a.as_pointer(), b.as_pointer(),) }) == std::cmp::Ordering::Less
    );
    return 0;
}
pub trait SImpl {
    fn operator_cmp(&self, o: Ptr<S>) -> std::cmp::Ordering;
    fn operator_eq(&self, o: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_cmp(&self, o: Ptr<S>) -> std::cmp::Ordering {
        if {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs < (*(*o.upgrade().deref()).v.borrow())
        } {
            return std::cmp::Ordering::Less;
        }
        if {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs > (*(*o.upgrade().deref()).v.borrow())
        } {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs == (*(*o.upgrade().deref()).v.borrow())
        };
    }
}
