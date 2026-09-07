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
pub fn operator_eq_0(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs == (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_ne_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs != (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_lt_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs < (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_gt_3(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs > (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_le_4(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs <= (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_ge_5(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs >= (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_lt_6(a: Ptr<S>, b: i32) -> bool {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs < (*b.borrow())
    };
}
pub fn operator_lt_7(a: i32, b: Ptr<S>) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs < (*(*b.upgrade().deref()).v.borrow())
    };
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
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_eq_0(_a, c.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_ne_1(_a, b.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_lt_2(_a, b.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = b.as_pointer();
            operator_gt_3(_a, a.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_le_4(_a, c.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_ge_5(_a, c.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = b.as_pointer();
            operator_lt_2(_a, a.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_lt_6(_a, 5)
        })
    );
    assert!(({ operator_lt_7(0, a.as_pointer(),) }));
    return 0;
}
