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
    let t: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(1)),
    }));
    let f: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(0)),
    }));
    assert!(({ SImpl::operator_not(&f.as_pointer(),) }));
    assert!(!({ SImpl::operator_not(&t.as_pointer(),) }));
    assert!(
        ({
            let _o: Ptr<S> = t.as_pointer();
            SImpl::operator_and(&t.as_pointer(), _o)
        })
    );
    assert!(!({ SImpl::operator_and(&t.as_pointer(), f.as_pointer(),) }));
    assert!(({ SImpl::operator_or(&t.as_pointer(), f.as_pointer(),) }));
    assert!(
        !({
            let _o: Ptr<S> = f.as_pointer();
            SImpl::operator_or(&f.as_pointer(), _o)
        })
    );
    return 0;
}
pub trait SImpl {
    fn operator_not(&self) -> bool;
    fn operator_and(&self, o: Ptr<S>) -> bool;
    fn operator_or(&self, o: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_not(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).v.borrow()) == 0);
    }
    fn operator_and(&self, o: Ptr<S>) -> bool {
        return ((*(*(*self).upgrade().deref()).v.borrow()) != 0)
            && ((*(*o.upgrade().deref()).v.borrow()) != 0);
    }
    fn operator_or(&self, o: Ptr<S>) -> bool {
        return ((*(*(*self).upgrade().deref()).v.borrow()) != 0)
            || ((*(*o.upgrade().deref()).v.borrow()) != 0);
    }
}
