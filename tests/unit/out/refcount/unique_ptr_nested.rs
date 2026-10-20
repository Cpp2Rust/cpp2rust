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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
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
    let mut o: Option<Value<Outer>> = Some(Rc::new(RefCell::new(Outer {
        inner: Some(Rc::new(RefCell::new(Inner { x: 10, y: 20 }))),
    })));
    {
        field!(o.as_pointer().with(|__s| __s.inner.clone()).as_pointer(), x)
            .with_mut(|__v| *__v = *__v + 5)
    };
    let mut sum: i32 = (o
        .as_pointer()
        .with(|__s| __s.inner.clone())
        .as_pointer()
        .with(|__s| __s.x)
        + o.as_pointer()
            .with(|__s| __s.inner.clone())
            .as_pointer()
            .with(|__s| __s.y));
    let mut a: Option<Value<i32>> = Some(Rc::new(RefCell::new(100)));
    let mut b: Option<Value<i32>> = Some(Rc::new(RefCell::new(0)));
    let __rhs = (a.as_pointer().read());
    b.as_pointer().write(__rhs);
    assert!(((sum + (b.as_pointer().read())) == 135));
    return 0;
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field_ptr!((*self), inner)
            .write(field!(_a0, inner).with_mut(|__v: &mut Option<Value<Inner>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
