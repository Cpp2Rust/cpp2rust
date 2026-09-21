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
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(S {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(2)),
    }));
    let s2: Value<S> = Rc::new(RefCell::new(S {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(2)),
    }));
    let s3: Value<S> = Rc::new(RefCell::new(S {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(3)),
    }));
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_eq_0(_x, s2.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_ne_1(_x, s3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = s1.as_pointer();
            operator_eq_0(_x, s3.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_lt_2(_x, s3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = s3.as_pointer();
            operator_lt_2(_x, s1.as_pointer())
        })
    );
    assert!((({ compare_3(s1.as_pointer(), s3.as_pointer(),) }) == -1_i32));
    assert!((({ compare_3(s3.as_pointer(), s1.as_pointer(),) }) == 1));
    assert!((({ compare_3(s1.as_pointer(), s2.as_pointer(),) }) == 0));
    return 0;
}
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({
        let _lhs = (*(*x.upgrade().deref()).a.borrow());
        _lhs == (*(*y.upgrade().deref()).a.borrow())
    }) && ({
        let _lhs = (*(*x.upgrade().deref()).b.borrow());
        _lhs == (*(*y.upgrade().deref()).b.borrow())
    });
}
pub fn operator_ne_1(x: Ptr<S>, y: Ptr<S>) -> bool {
    return !({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_eq_0(_x, _y)
    });
}
pub fn operator_lt_2(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({
        let _lhs = (*(*x.upgrade().deref()).a.borrow());
        _lhs < (*(*y.upgrade().deref()).a.borrow())
    }) || (({
        let _lhs = (*(*x.upgrade().deref()).a.borrow());
        _lhs == (*(*y.upgrade().deref()).a.borrow())
    }) && ({
        let _lhs = (*(*x.upgrade().deref()).b.borrow());
        _lhs < (*(*y.upgrade().deref()).b.borrow())
    }));
}
pub fn compare_3(x: Ptr<S>, y: Ptr<S>) -> i32 {
    if ({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_lt_2(_x, _y)
    }) {
        return -1_i32;
    }
    if ({
        let _x: Ptr<S> = (y).clone();
        let _y: Ptr<S> = (x).clone();
        operator_lt_2(_x, _y)
    }) {
        return 1;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
