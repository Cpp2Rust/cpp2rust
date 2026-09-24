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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let outer: Value<_> = Rc::new(RefCell::new(
        (|y: i32| {
            let y: Value<i32> = Rc::new(RefCell::new(y));
            let inner: Value<_> = Rc::new(RefCell::new(
                (|z: i32| {
                    let z: Value<i32> = Rc::new(RefCell::new(z));
                    return (((*x.borrow()) + (*y.borrow())) + (*z.borrow()));
                }),
            ));
            return ({ (*inner.borrow_mut())(1) });
        }),
    ));
    assert!((({ (*outer.borrow_mut())(20,) }) == 31));
    (*x.borrow_mut()) = 100;
    assert!((({ (*outer.borrow_mut())(20,) }) == 121));
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(5)),
    }));
    assert!((({ SImpl::nested_this(&s.as_pointer(),) }) == 26));
    return 0;
}
pub trait SImpl {
    fn nested_this(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn nested_this(&self) -> i32 {
        let outer: Value<_> = Rc::new(RefCell::new(
            (|y: i32| {
                let y: Value<i32> = Rc::new(RefCell::new(y));
                let inner: Value<_> = Rc::new(RefCell::new(
                    (|z: i32| {
                        let z: Value<i32> = Rc::new(RefCell::new(z));
                        return (((*self.v.borrow()) + (*y.borrow())) + (*z.borrow()));
                    }),
                ));
                return ({ (*inner.borrow_mut())(1) });
            }),
        ));
        return ({ (*outer.borrow_mut())(20) });
    }
}
pub fn __cpp2rust_init_globals() {}
