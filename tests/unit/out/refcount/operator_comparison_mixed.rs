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
pub fn operator_eq_0(a: i32, b: Ptr<S>) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs == (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_ne_1(a: i32, b: Ptr<S>) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs != (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_lt_2(a: i32, b: Ptr<S>) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs < (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_gt_3(a: f64, b: Ptr<S>) -> bool {
    let a: Value<f64> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs > ((*(*b.upgrade().deref()).v.borrow()) as f64)
    };
}
pub fn operator_le_4(a: i64, b: Ptr<S>) -> bool {
    let a: Value<i64> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs <= ((*(*b.upgrade().deref()).v.borrow()) as i64)
    };
}
pub fn operator_ge_5(a: Ptr<u8>, b: Ptr<S>) -> bool {
    let a: Value<Ptr<u8>> = Rc::new(RefCell::new(a));
    return {
        let _lhs = ((((*a.borrow()).read()) as i32) - (('0' as u8) as i32));
        _lhs >= (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_lt_6(a: Ptr<S>, b: i32) -> bool {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return {
        let _lhs = ((*(*a.upgrade().deref()).v.borrow()) + 1);
        _lhs < (*b.borrow())
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(5)),
    }));
    let cs: Ptr<S> = s.as_pointer();
    assert!(({ SImpl::operator_eq(&cs, 5,) }));
    assert!(({ SImpl::operator_ne(&cs, 4,) }));
    assert!(({ SImpl::operator_lt(&cs, 6,) }));
    assert!(({ SImpl::operator_gt(&cs, 4.5E+0,) }));
    assert!(({ SImpl::operator_le(&cs, 5_i64,) }));
    assert!(({ SImpl::operator_ge(&cs, Ptr::from_string_literal(b"3"),) }));
    assert!(({ operator_eq_0(5, s.as_pointer(),) }));
    assert!(({ operator_ne_1(4, s.as_pointer(),) }));
    assert!(({ operator_lt_2(4, s.as_pointer(),) }));
    assert!(({ operator_gt_3(5.5E+0, s.as_pointer(),) }));
    assert!(({ operator_le_4(5_i64, s.as_pointer(),) }));
    assert!(({ operator_ge_5(Ptr::from_string_literal(b"7"), s.as_pointer(),) }));
    assert!(
        ({
            let _a: Ptr<S> = s.as_pointer();
            operator_lt_6(_a, 7)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = s.as_pointer();
            operator_lt_6(_a, 6)
        })
    );
    return 0;
}
pub trait SImpl {
    fn operator_eq(&self, o: i32) -> bool;
    fn operator_ne(&self, o: i32) -> bool;
    fn operator_lt(&self, o: i32) -> bool;
    fn operator_gt(&self, o: f64) -> bool;
    fn operator_le(&self, o: i64) -> bool;
    fn operator_ge(&self, o: Ptr<u8>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, o: i32) -> bool {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return ((*(*(*self).upgrade().deref()).v.borrow()) == (*o.borrow()));
    }
    fn operator_ne(&self, o: i32) -> bool {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return ((*(*(*self).upgrade().deref()).v.borrow()) != (*o.borrow()));
    }
    fn operator_lt(&self, o: i32) -> bool {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return ((*(*(*self).upgrade().deref()).v.borrow()) < (*o.borrow()));
    }
    fn operator_gt(&self, o: f64) -> bool {
        let o: Value<f64> = Rc::new(RefCell::new(o));
        return (((*(*(*self).upgrade().deref()).v.borrow()) as f64) > (*o.borrow()));
    }
    fn operator_le(&self, o: i64) -> bool {
        let o: Value<i64> = Rc::new(RefCell::new(o));
        return (((*(*(*self).upgrade().deref()).v.borrow()) as i64) <= (*o.borrow()));
    }
    fn operator_ge(&self, o: Ptr<u8>) -> bool {
        let o: Value<Ptr<u8>> = Rc::new(RefCell::new(o));
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs >= ((((*o.borrow()).read()) as i32) - (('0' as u8) as i32))
        };
    }
}
