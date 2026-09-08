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
pub fn operator_not_0(a: Ptr<S>) -> bool {
    return ((*(*a.upgrade().deref()).v.borrow()) == 0);
}
pub fn operator_and_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ((*(*a.upgrade().deref()).v.borrow()) != 0)
        && ((*(*b.upgrade().deref()).v.borrow()) != 0);
}
pub fn operator_or_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ((*(*a.upgrade().deref()).v.borrow()) != 0)
        || ((*(*b.upgrade().deref()).v.borrow()) != 0);
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
    assert!(
        ({
            let _a: Ptr<S> = f.as_pointer();
            operator_not_0(_a)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = t.as_pointer();
            operator_not_0(_a)
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = t.as_pointer();
            let _b: Ptr<S> = t.as_pointer();
            operator_and_1(_a, _b)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = t.as_pointer();
            operator_and_1(_a, f.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = t.as_pointer();
            operator_or_2(_a, f.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = f.as_pointer();
            let _b: Ptr<S> = f.as_pointer();
            operator_or_2(_a, _b)
        })
    );
    return 0;
}
