extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Record, ByteRepr, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub inner: Option<Value<Inner>>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            inner: field!(_a0, inner).with_mut(|__v: &mut Option<Value<Inner>>| __v.take()),
        }
    }
}
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
    (*{ (*(*o.borrow()).as_ref().unwrap().borrow()).inner.clone() }
        .as_ref()
        .unwrap()
        .borrow_mut())
    .x += 5;
    let mut sum: i32 = ({
        (*{ (*(*o.borrow()).as_ref().unwrap().borrow()).inner.clone() }
            .as_ref()
            .unwrap()
            .borrow())
        .x
    } + {
        (*{ (*(*o.borrow()).as_ref().unwrap().borrow()).inner.clone() }
            .as_ref()
            .unwrap()
            .borrow())
        .y
    });
    let a: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(100)))));
    let b: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
    let __rhs = (*(*a.borrow()).as_ref().unwrap().borrow());
    (*(*b.borrow()).as_ref().unwrap().borrow_mut()) = __rhs;
    assert!(((sum + (*(*b.borrow()).as_ref().unwrap().borrow())) == 135));
    return 0;
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        (field_ptr!((*self), inner) as Ptr<Option<Value<Inner>>>)
            .write(field!(_a0, inner).with_mut(|__v: &mut Option<Value<Inner>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
