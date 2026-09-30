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
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
pub struct Table {}
impl Table {
    pub fn operator_index(i: i32) -> Ptr<i32> {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return (table_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((*i.borrow()));
    }
}
thread_local!(
    pub static table_0: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8, 9])));
);
#[derive(Record, VaArg, FnPtrArg)]
pub struct S {
    #[offset(0)]
    pub data: Value<Box<[i32]>>,
    #[offset(12)]
    pub inner: Inner,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
                |__i: usize| (*self.data.borrow())[(__i) as usize],
            )))),
            inner: { self.inner.clone() },
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            data: Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>())),
            inner: <Inner>::default(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..12]);
        self.inner.to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..12]))),
            inner: <Inner>::from_bytes(&buf[12..16]),
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
    assert!((({ SImpl::operator_deref(&s.as_pointer(),) }).with(|__s: &Inner| __s.x) == 9));
    ({ SImpl::operator_deref(&s.as_pointer()) }).with_mut(|__s: &mut Inner| __s.x = 10);
    assert!(
        ({
            (*({ SImpl::operator_arrow(&s.as_pointer()) })
                .upgrade()
                .deref())
            .x
        } == 10)
    );
    (*({ SImpl::operator_arrow(&s.as_pointer()) })
        .upgrade()
        .deref_mut())
    .x = 11;
    assert!(({ (*s.borrow()).inner.x } == 11));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(({ SImpl::operator_addr(&s.as_pointer()) })));
    assert!((((*p.borrow()).read()) == 1));
    (*p.borrow()).write(5);
    assert!(((*(*s.borrow()).data.borrow())[(0) as usize] == 5));
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
    fn operator_index_1(&self, i: i32) -> Ptr<i32> {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return ((*self).with(|__s: &S| __s.data.as_pointer()) as Ptr<i32>).offset((*i.borrow()));
    }
    fn operator_index_2(&self, i: i32) -> Ptr<i32> {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return ((*self).with(|__s: &S| __s.data.as_pointer()) as Ptr<i32>).offset((*i.borrow()));
    }
    fn operator_deref(&self) -> Ptr<Inner> {
        return field_ptr!((*self), inner);
    }
    fn operator_arrow(&self) -> Ptr<Inner> {
        return (field_ptr!((*self), inner));
    }
    fn operator_addr(&self) -> Ptr<i32> {
        return (((*self).with(|__s: &S| __s.data.as_pointer()) as Ptr<i32>).offset(0));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = table_0.with(|_| ());
}
