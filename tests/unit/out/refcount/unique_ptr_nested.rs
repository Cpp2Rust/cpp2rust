extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Record, Default)]
pub struct Outer {
    #[offset(0)]
    pub inner: Option<Value<Inner>>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            inner: { _a0.with_mut(|__s: &mut Outer| __s.inner.take()) },
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for Outer {}
=======
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.inner.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: <Option<Value<Inner>>>::from_bytes(&buf[0..8]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Option<Value<Outer>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new({
        let __tmp_0: Value<Outer> = Rc::new(RefCell::new(Outer {
            inner: Some(Rc::new(RefCell::new(Inner { x: 10, y: 20 }))),
        }));
        Outer::move_from({ __tmp_0.as_pointer() })
    })))));
    (*(*(*o.borrow()).as_ref().unwrap().borrow())
        .inner
        .as_ref()
        .unwrap()
        .borrow_mut())
    .x += 5;
    let sum: Value<i32> = Rc::new(RefCell::new(
        ({
            (*(*(*o.borrow()).as_ref().unwrap().borrow())
                .inner
                .as_ref()
                .unwrap()
                .borrow())
            .x
        } + {
            (*(*(*o.borrow()).as_ref().unwrap().borrow())
                .inner
                .as_ref()
                .unwrap()
                .borrow())
            .y
        }),
    ));
    let a: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(100)))));
    let b: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let __rhs = (*(*a.borrow()).as_ref().unwrap().borrow());
    (*(*b.borrow()).as_ref().unwrap().borrow_mut()) = __rhs;
    assert!((((*sum.borrow()) + (*(*b.borrow()).as_ref().unwrap().borrow())) == 135));
    return 0;
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        ((field_ptr!((*self), inner) as Ptr<Option<Value<Inner>>>) as Ptr<Option<Value<Inner>>>)
            .write(_a0.with_mut(|__s: &mut Outer| __s.inner.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
