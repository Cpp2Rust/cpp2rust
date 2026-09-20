extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Handler {
    pub tag: Value<i32>,
    pub cb: Value<FnPtr<fn(i32) -> i32>>,
}
impl Clone for Handler {
    fn clone(&self) -> Self {
        let __this: Value<Handler> = Rc::new(RefCell::new(Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            cb: Rc::new(RefCell::new((*self.cb.borrow()).clone())),
        }));
        let this: Ptr<Handler> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Handler {
    fn default() -> Self {
        Handler {
            tag: <Value<i32>>::default(),
            cb: Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null())),
        }
    }
}
impl ByteRepr for Handler {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.cb.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            cb: Rc::new(RefCell::new(<FnPtr<fn(i32) -> i32>>::from_bytes(
                &buf[8..16],
            ))),
        }
    }
}
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn negate_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return -(*x.borrow());
}
#[derive(Clone, ByteRepr, Default)]
pub struct S {}
impl S {
    pub fn pick_i32(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + 1);
    }
    pub fn pick_i64(x: i64) -> i32 {
        let x: Value<i64> = Rc::new(RefCell::new(x));
        return (((*x.borrow()) as i32) + 2);
    }
    pub fn solo(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + 3);
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new((FnPtr::<fn(i32) -> i32>::new(pick))));
    let p2: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(solo)));
    assert!((({ (*(*p1.borrow()))(5,) }) == 6));
    assert!((({ (*(*p2.borrow()))(5,) }) == 8));
    assert!((({ S::pick(5_i64,) }) == 7));
    let h3: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: Rc::new(RefCell::new(3)),
        cb: Rc::new(RefCell::new((FnPtr::<fn(i32) -> i32>::new(pick)))),
    }));
    assert!((({ (*(*(*h3.borrow()).cb.borrow()))(1,) }) == 2));
    let h1: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: Rc::new(RefCell::new(1)),
        cb: Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(double_it_0))),
    }));
    let h2: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: Rc::new(RefCell::new(2)),
        cb: Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(negate_1))),
    }));
    assert!(!((*(*h1.borrow()).cb.borrow()).is_null()));
    assert!((({ (*(*(*h1.borrow()).cb.borrow()))(5,) }) == 10));
    assert!((({ (*(*(*h2.borrow()).cb.borrow()))(7,) }) == -7_i32));
    (*(*h1.borrow()).cb.borrow_mut()) = FnPtr::<fn(i32) -> i32>::new(negate_1);
    assert!((({ (*(*(*h1.borrow()).cb.borrow()))(3,) }) == -3_i32));
    assert!({
        let _lhs = (*(*h1.borrow()).cb.borrow()).clone();
        _lhs == (*(*h2.borrow()).cb.borrow()).clone()
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
