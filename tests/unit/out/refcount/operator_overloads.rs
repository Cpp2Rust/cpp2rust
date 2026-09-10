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
pub fn operator_div_0(a: Ptr<S>, b: Ptr<S>) -> i32 {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs / (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_div_1(a: S, b: i32) -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return (((*(*a.borrow()).v.borrow()) / (*b.borrow())) + 1);
}
pub fn operator_rem_2(a: S, b: S) -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(a));
    let b: Value<S> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).v.borrow()) % (*(*b.borrow()).v.borrow()));
}
pub fn operator_rem_3(a: Ptr<S>, b: i32) -> i32 {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ({
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs % (*b.borrow())
    } + 1);
}
pub fn operator_eq_4(a: i32, b: S) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<S> = Rc::new(RefCell::new(b));
    return if ((*a.borrow()) == (*(*b.borrow()).v.borrow())) {
        4
    } else {
        0
    };
}
pub fn operator_eq_5(a: i64, b: Ptr<S>) -> i32 {
    let a: Value<i64> = Rc::new(RefCell::new(a));
    return if {
        let _lhs = (*a.borrow());
        _lhs == ((*(*b.upgrade().deref()).v.borrow()) as i64)
    } {
        5
    } else {
        0
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(6)),
    }));
    let t: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(4)),
    }));
    assert!((({ SImpl::operator_eq_i32_const(&s.as_pointer(), 6,) }) == 1));
    assert!((({ SImpl::operator_eq_i64_const(&s.as_pointer(), 6_i64,) }) == 2));
    assert!((({ SImpl::operator_eq_f64_const(&s.as_pointer(), 6.0E+0,) }) == 3));
    assert!((({ SImpl::operator_eq_i32_const(&s.as_pointer(), 7,) }) == 0));
    assert!((({ SImpl::operator_add(&s.as_pointer(), t.as_pointer(),) }) == 10));
    assert!((({ SImpl::operator_sub(&s.as_pointer(), (*t.borrow()).clone(),) }) == 2));
    assert!((({ SImpl::operator_mul_pconstS_const(&s.as_pointer(), t.as_pointer(),) }) == 24));
    assert!((({ SImpl::operator_mul_i32_const(&s.as_pointer(), 2,) }) == 13));
    assert!(
        (({
            let _a: Ptr<S> = s.as_pointer();
            operator_div_0(_a, t.as_pointer())
        }) == 1)
    );
    assert!(
        (({
            let _a: S = (*s.borrow()).clone();
            operator_div_1(_a, 4)
        }) == 2)
    );
    assert!(
        (({
            let _a: S = (*s.borrow()).clone();
            operator_rem_2(_a, (*t.borrow()).clone())
        }) == 2)
    );
    assert!(
        (({
            let _a: Ptr<S> = s.as_pointer();
            operator_rem_3(_a, 4)
        }) == 3)
    );
    assert!((({ operator_eq_4(6, (*s.borrow()).clone(),) }) == 4));
    assert!((({ operator_eq_5(6_i64, s.as_pointer(),) }) == 5));
    return 0;
}
pub trait SImpl {
    fn operator_eq_i32_const(&self, o: i32) -> i32;
    fn operator_eq_i64_const(&self, o: i64) -> i32;
    fn operator_eq_f64_const(&self, o: f64) -> i32;
    fn operator_add(&self, o: Ptr<S>) -> i32;
    fn operator_sub(&self, o: S) -> i32;
    fn operator_mul_pconstS_const(&self, o: Ptr<S>) -> i32;
    fn operator_mul_i32_const(&self, o: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_eq_i32_const(&self, o: i32) -> i32 {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return if ((*(*(*self).upgrade().deref()).v.borrow()) == (*o.borrow())) {
            1
        } else {
            0
        };
    }
    fn operator_eq_i64_const(&self, o: i64) -> i32 {
        let o: Value<i64> = Rc::new(RefCell::new(o));
        return if (((*(*(*self).upgrade().deref()).v.borrow()) as i64) == (*o.borrow())) {
            2
        } else {
            0
        };
    }
    fn operator_eq_f64_const(&self, o: f64) -> i32 {
        let o: Value<f64> = Rc::new(RefCell::new(o));
        return if (((*(*(*self).upgrade().deref()).v.borrow()) as f64) == (*o.borrow())) {
            3
        } else {
            0
        };
    }
    fn operator_add(&self, o: Ptr<S>) -> i32 {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs + (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_sub(&self, o: S) -> i32 {
        let o: Value<S> = Rc::new(RefCell::new(o));
        return ((*(*(*self).upgrade().deref()).v.borrow()) - (*(*o.borrow()).v.borrow()));
    }
    fn operator_mul_pconstS_const(&self, o: Ptr<S>) -> i32 {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs * (*(*o.upgrade().deref()).v.borrow())
        };
    }
    fn operator_mul_i32_const(&self, o: i32) -> i32 {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return (((*(*(*self).upgrade().deref()).v.borrow()) * (*o.borrow())) + 1);
    }
}
