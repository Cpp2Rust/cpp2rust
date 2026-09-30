extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct X {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for X {
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
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Y {
    #[offset(0)]
    pub x: X,
    #[offset(8)]
    pub p: Ptr<X>,
}
impl ByteRepr for Y {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.p.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <X>::from_bytes(&buf[0..4]),
            p: <Ptr<X>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(5));
    let x2: Value<i32> = Rc::new(RefCell::new((*x1.borrow())));
    let x3: Value<i32> = Rc::new(RefCell::new(((*x1.borrow()) + 5)));
    let x4: Value<i32> = Rc::new(RefCell::new(((*x3.borrow()) + (*x2.borrow()))));
    (*x1.borrow_mut()) = 5;
    (*x2.borrow_mut()) = (*x1.borrow());
    (*x3.borrow_mut()) = ((*x1.borrow()) + 5);
    (*x4.borrow_mut()) = ((*x3.borrow()) + (*x2.borrow()));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    (*p1.borrow_mut()) = (x2.as_pointer());
    let __rhs = (*x1.borrow());
    (*p1.borrow()).write(__rhs);
    let __rhs = (((*x1.borrow()) + (*x4.borrow())) + 1);
    (*p1.borrow()).write(__rhs);
    let x5: Value<i32> = Rc::new(RefCell::new(((*p1.borrow()).read())));
    let x6: Value<i32> = Rc::new(RefCell::new(
        ({
            let _lhs = ((*p1.borrow()).read());
            _lhs + (*x3.borrow())
        } + 5),
    ));
    let r: Ptr<i32> = x1.as_pointer();
    r.write(5);
    let __rhs = (((*p1.borrow()).read()) + 5);
    r.write(__rhs);
    let x7: Value<i32> = Rc::new(RefCell::new((r.read())));
    let x8: Value<i32> = Rc::new(RefCell::new(
        ({
            let _lhs = (r.read());
            _lhs + (*x1.borrow())
        } + 5),
    ));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((r).clone()));
    let x: Value<X> = Rc::new(RefCell::new(X { x: 1 }));
    let y: Value<Y> = Rc::new(RefCell::new(Y {
        x: X { x: 0 },
        p: (x.as_pointer()),
    }));
    (*y.borrow_mut()).x.x = 5;
    ({ YImpl::foo(&y.as_pointer()) }).with_mut(|__s: &mut X| __s.x = 1);
    { (*y.borrow()).p.clone() }.with_mut(|__s: &mut X| __s.x = 10);
    let p3: Value<Ptr<Y>> = Rc::new(RefCell::new((y.as_pointer())));
    (*p3.borrow())
        .with(|__s: &Y| (__s.p).clone())
        .with_mut(|__s: &mut X| __s.x = 100);
    ({ YImpl::ptr(&y.as_pointer()) }).with_mut(|__s: &mut X| __s.x = 1);
    ({ YImpl::ptr(&y.as_pointer()) }).with_mut(|__s: &mut X| __s.x = 50);
    assert!(({ (*x.borrow()).x } == 100));
    return 0;
}
pub trait YImpl {
    fn foo(&self) -> Ptr<X>;
    fn ptr(&self) -> Ptr<X>;
}
impl YImpl for Ptr<Y> {
    fn foo(&self) -> Ptr<X> {
        return field_ptr!((*self), x);
    }
    fn ptr(&self) -> Ptr<X> {
        return (field_ptr!((*self), x));
    }
}
pub fn __cpp2rust_init_globals() {}
