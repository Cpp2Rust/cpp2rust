extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn operator_div_0(a: Ptr<S>, b: Ptr<S>) -> i32 {
    return {
        let _lhs = a.with(|__s| __s.v);
        _lhs / b.with(|__s| __s.v)
    };
}
pub fn operator_div_1(a: S, b: i32) -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return (({ (*a.borrow()).v } / (*b.borrow())) + 1);
}
pub fn operator_rem_2(a: S, b: S) -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(a));
    let b: Value<S> = Rc::new(RefCell::new(b));
    return ({ (*a.borrow()).v } % { (*b.borrow()).v });
}
pub fn operator_rem_3(a: Ptr<S>, b: i32) -> i32 {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ({
        let _lhs = a.with(|__s| __s.v);
        _lhs % (*b.borrow())
    } + 1);
}
pub fn operator_eq_4(a: i32, b: S) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<S> = Rc::new(RefCell::new(b));
    return if ((*a.borrow()) == { (*b.borrow()).v }) {
        4
    } else {
        0
    };
}
pub fn operator_eq_5(a: i64, b: Ptr<S>) -> i32 {
    let a: Value<i64> = Rc::new(RefCell::new(a));
    return if {
        let _lhs = (*a.borrow());
        _lhs == (b.with(|__s| __s.v) as i64)
    } {
        5
    } else {
        0
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 6 }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
    assert!((({ SImpl::operator_eq_1(&s.as_pointer(), 6,) }) == 1));
    assert!((({ SImpl::operator_eq_2(&s.as_pointer(), 6_i64,) }) == 2));
    assert!((({ SImpl::operator_eq_3(&s.as_pointer(), 6.0E+0,) }) == 3));
    assert!((({ SImpl::operator_eq_1(&s.as_pointer(), 7,) }) == 0));
    assert!((({ SImpl::operator_add(&s.as_pointer(), t.as_pointer(),) }) == 10));
    assert!((({ SImpl::operator_sub(&s.as_pointer(), (*t.borrow()).clone(),) }) == 2));
    assert!((({ SImpl::operator_mul_6(&s.as_pointer(), t.as_pointer(),) }) == 24));
    assert!((({ SImpl::operator_mul_7(&s.as_pointer(), 2,) }) == 13));
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
    fn operator_eq_1(&self, o: i32) -> i32;
    fn operator_eq_2(&self, o: i64) -> i32;
    fn operator_eq_3(&self, o: f64) -> i32;
    fn operator_add(&self, o: Ptr<S>) -> i32;
    fn operator_sub(&self, o: S) -> i32;
    fn operator_mul_6(&self, o: Ptr<S>) -> i32;
    fn operator_mul_7(&self, o: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_eq_1(&self, o: i32) -> i32 {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return if ((*self).with(|__s| __s.v) == (*o.borrow())) {
            1
        } else {
            0
        };
    }
    fn operator_eq_2(&self, o: i64) -> i32 {
        let o: Value<i64> = Rc::new(RefCell::new(o));
        return if (((*self).with(|__s| __s.v) as i64) == (*o.borrow())) {
            2
        } else {
            0
        };
    }
    fn operator_eq_3(&self, o: f64) -> i32 {
        let o: Value<f64> = Rc::new(RefCell::new(o));
        return if (((*self).with(|__s| __s.v) as f64) == (*o.borrow())) {
            3
        } else {
            0
        };
    }
    fn operator_add(&self, o: Ptr<S>) -> i32 {
        return {
            let _lhs = (*self).with(|__s| __s.v);
            _lhs + o.with(|__s| __s.v)
        };
    }
    fn operator_sub(&self, o: S) -> i32 {
        let o: Value<S> = Rc::new(RefCell::new(o));
        return ((*self).with(|__s| __s.v) - { (*o.borrow()).v });
    }
    fn operator_mul_6(&self, o: Ptr<S>) -> i32 {
        return {
            let _lhs = (*self).with(|__s| __s.v);
            _lhs * o.with(|__s| __s.v)
        };
    }
    fn operator_mul_7(&self, o: i32) -> i32 {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return (((*self).with(|__s| __s.v) * (*o.borrow())) + 1);
    }
}
pub fn __cpp2rust_init_globals() {}
