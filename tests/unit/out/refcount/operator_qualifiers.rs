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
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(10)),
    }));
    let cs: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(10)),
    }));
    let vs: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(10)),
    }));
    assert!((({ SImpl::operator_add_i32(&s.as_pointer(), 1,) }) == 11));
    assert!((({ SImpl::operator_add_i32_const(&cs.as_pointer(), 1,) }) == 12));
    assert!((({ SImpl::operator_add_i32_volatile(&vs.as_pointer(), 1,) }) == 13));
    assert!((({ SImpl::operator_sub_i32_lref(&s.as_pointer(), 1,) }) == 9));
    assert!(
        (({
            SImpl::operator_sub_i32_rref(
                &S {
                    v: Rc::new(RefCell::new(10)),
                },
                1,
            )
        }) == 8)
    );
    assert!((({ SImpl::operator_mul_i32_const_lref(&s.as_pointer(), 3,) }) == 30));
    assert!((({ SImpl::operator_mul_i32_const_lref(&cs.as_pointer(), 3,) }) == 30));
    assert!(
        (({
            SImpl::operator_mul_i32_const_rref(
                &S {
                    v: Rc::new(RefCell::new(10)),
                },
                3,
            )
        }) == 60)
    );
    assert!((({ SImpl::operator_index_i32_lref(&s.as_pointer(), 2,) }) == 12));
    assert!((({ SImpl::operator_index_i32_const_lref(&cs.as_pointer(), 2,) }) == 112));
    return 0;
}
pub trait SImpl {
    fn operator_add_i32(&self, a: i32) -> i32;
    fn operator_add_i32_const(&self, a: i32) -> i32;
    fn operator_add_i32_volatile(&self, a: i32) -> i32;
    fn operator_sub_i32_lref(&self, a: i32) -> i32;
    fn operator_sub_i32_rref(&self, a: i32) -> i32;
    fn operator_mul_i32_const_lref(&self, a: i32) -> i32;
    fn operator_mul_i32_const_rref(&self, a: i32) -> i32;
    fn operator_index_i32_lref(&self, i: i32) -> i32;
    fn operator_index_i32_const_lref(&self, i: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_add_i32(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow()));
    }
    fn operator_add_i32_const(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow())) + 1);
    }
    fn operator_add_i32_volatile(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow())) + 2);
    }
    fn operator_sub_i32_lref(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) - (*a.borrow()));
    }
    fn operator_sub_i32_rref(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) - (*a.borrow())) - 1);
    }
    fn operator_mul_i32_const_lref(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) * (*a.borrow()));
    }
    fn operator_mul_i32_const_rref(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) * (*a.borrow())) * 2);
    }
    fn operator_index_i32_lref(&self, i: i32) -> i32 {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return ((*(*(*self).upgrade().deref()).v.borrow()) + (*i.borrow()));
    }
    fn operator_index_i32_const_lref(&self, i: i32) -> i32 {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*i.borrow())) + 100);
    }
}
