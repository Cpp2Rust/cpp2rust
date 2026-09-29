extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
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
    __cpp2rust_init_globals();
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
    assert!((({ SImpl::operator_add_1(&s.as_pointer(), 1,) }) == 11));
    assert!((({ SImpl::operator_add_2(&cs.as_pointer(), 1,) }) == 12));
    assert!((({ SImpl::operator_add_3(&vs.as_pointer(), 1,) }) == 13));
    assert!((({ SImpl::operator_sub_4(&s.as_pointer(), 1,) }) == 9));
    assert!(
        (({
            SImpl::operator_sub_5(
                &Rc::new(RefCell::new(S {
                    v: Rc::new(RefCell::new(10)),
                }))
                .as_pointer(),
                1,
            )
        }) == 8)
    );
    assert!((({ SImpl::operator_mul_6(&s.as_pointer(), 3,) }) == 30));
    assert!((({ SImpl::operator_mul_6(&cs.as_pointer(), 3,) }) == 30));
    assert!(
        (({
            SImpl::operator_mul_7(
                &Rc::new(RefCell::new(S {
                    v: Rc::new(RefCell::new(10)),
                }))
                .as_pointer(),
                3,
            )
        }) == 60)
    );
    assert!((({ SImpl::operator_index_8(&s.as_pointer(), 2,) }) == 12));
    assert!((({ SImpl::operator_index_9(&cs.as_pointer(), 2,) }) == 112));
    return 0;
}
pub trait SImpl {
    fn operator_add_1(&self, a: i32) -> i32;
    fn operator_add_2(&self, a: i32) -> i32;
    fn operator_add_3(&self, a: i32) -> i32;
    fn operator_sub_4(&self, a: i32) -> i32;
    fn operator_sub_5(&self, a: i32) -> i32;
    fn operator_mul_6(&self, a: i32) -> i32;
    fn operator_mul_7(&self, a: i32) -> i32;
    fn operator_index_8(&self, i: i32) -> i32;
    fn operator_index_9(&self, i: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow()));
    }
    fn operator_add_2(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow())) + 1);
    }
    fn operator_add_3(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow())) + 2);
    }
    fn operator_sub_4(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) - (*a.borrow()));
    }
    fn operator_sub_5(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) - (*a.borrow())) - 1);
    }
    fn operator_mul_6(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) * (*a.borrow()));
    }
    fn operator_mul_7(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return (((*(*(*self).upgrade().deref()).v.borrow()) * (*a.borrow())) * 2);
    }
    fn operator_index_8(&self, i: i32) -> i32 {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return ((*(*(*self).upgrade().deref()).v.borrow()) + (*i.borrow()));
    }
    fn operator_index_9(&self, i: i32) -> i32 {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*i.borrow())) + 100);
    }
}
pub fn __cpp2rust_init_globals() {}
