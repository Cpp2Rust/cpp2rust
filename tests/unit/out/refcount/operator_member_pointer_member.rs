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
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Table {}
impl Table {
    pub fn operator_index(mut i: i32) -> Ptr<i32> {
        return (table_0.with(|v| v.as_pointer()) as Ptr<i32>).offset(i);
    }
}
thread_local!(
    pub static table_0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8, 9])));
);
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    #[byte_size(12)]
    pub data: Value<Box<[i32]>>,
    #[offset(12)]
    #[byte_size(4)]
    pub inner: Inner,
}
impl Default for S {
    fn default() -> Self {
        S {
            data: Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>())),
            inner: <Inner>::default(),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        data: Rc::new(RefCell::new(Box::new([1, 2, 3]))),
        inner: Inner { x: 9 },
    }));
    assert!(((({ SImpl::operator_index_1(&s.as_pointer(), 1,) }).read()) == 2));
    ({ SImpl::operator_index_1(&s.as_pointer(), 1) }).write(20);
    assert!(((({ SImpl::operator_index_1(&s.as_pointer(), 1,) }).read()) == 20));
    let cs: Ptr<S> = s.as_pointer();
    assert!(((({ SImpl::operator_index_2(&cs, 2,) }).read()) == 3));
    assert!((({ SImpl::operator_deref(&s.as_pointer(),) }).with(|__s| (__s).x) == 9));
    field!(({ SImpl::operator_deref(&s.as_pointer(),) }), x).write(10);
    assert!((({ SImpl::operator_arrow(&s.as_pointer(),) }).with(|__s| __s.x) == 10));
    field!(({ SImpl::operator_arrow(&s.as_pointer(),) }), x).write(11);
    assert!(({ (*s.borrow()).inner.x } == 11));
    let mut p: Ptr<i32> = ({ SImpl::operator_addr(&s.as_pointer()) });
    assert!(((p.read()) == 1));
    p.write(5);
    assert!(((elem!((array_field_ptr!(s.as_pointer(), data) as Ptr::<i32>), 0).read()) == 5));
    let t: Value<Table> = Rc::new(RefCell::new(<Table>::default()));
    assert!(((({ Table::operator_index(1,) }).read()) == 8));
    ({ Table::operator_index(1) }).write(80);
    assert!(
        (({
            let __idx = (1) as usize;
            table_0.with(|rc| rc.borrow()[__idx])
        }) == 80)
    );
    return 0;
}
pub trait SImpl {
    fn operator_index_1(&self, i: i32) -> Ptr<i32>;
    fn operator_index_2(&self, i: i32) -> Ptr<i32>;
    fn operator_deref(&self) -> Ptr<Inner>;
    fn operator_arrow(&self) -> Ptr<Inner>;
    fn operator_addr(&self) -> Ptr<i32>;
}
impl SImpl for Ptr<S> {
    fn operator_index_1(&self, mut i: i32) -> Ptr<i32> {
        return (array_field_ptr!((*self), data) as Ptr<i32>).offset((i) as isize);
    }
    fn operator_index_2(&self, mut i: i32) -> Ptr<i32> {
        return (array_field_ptr!((*self), data) as Ptr<i32>).offset((i) as isize);
    }
    fn operator_deref(&self) -> Ptr<Inner> {
        return field_ptr!((*self), inner);
    }
    fn operator_arrow(&self) -> Ptr<Inner> {
        return (field_ptr!((*self), inner));
    }
    fn operator_addr(&self) -> Ptr<i32> {
        return ((array_field_ptr!((*self), data) as Ptr<i32>).offset((0) as isize));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = table_0.with(|_| ());
}
