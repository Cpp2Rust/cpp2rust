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
pub struct X {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Y {
    #[offset(0)]
    #[byte_size(4)]
    pub x: X,
    #[offset(8)]
    #[byte_size(8)]
    pub p: Ptr<X>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(5));
    let x2: Value<i32> = Rc::new(RefCell::new((*x1.borrow())));
    let mut x3: i32 = ((*x1.borrow()) + 5);
    let mut x4: i32 = (x3 + (*x2.borrow()));
    (*x1.borrow_mut()) = 5;
    (*x2.borrow_mut()) = (*x1.borrow());
    x3 = ((*x1.borrow()) + 5);
    x4 = (x3 + (*x2.borrow()));
    let mut p1: Ptr<i32> = (x1.as_pointer());
    p1 = (x2.as_pointer());
    p1.write({ (*x1.borrow()) });
    p1.write({ (((*x1.borrow()) + x4) + 1) });
    let mut x5: i32 = (p1.read());
    let mut x6: i32 = (({ (p1.read()) } + { x3 }) + 5);
    let r: Ptr<i32> = x1.as_pointer();
    r.write(5);
    r.write({ ((p1.read()) + 5) });
    let mut x7: i32 = (r.read());
    let mut x8: i32 = (({ (r.read()) } + { (*x1.borrow()) }) + 5);
    let mut p2: Ptr<i32> = (r).clone();
    let x: Value<X> = Rc::new(RefCell::new(X { x: 1 }));
    let y: Value<Y> = Rc::new(RefCell::new(Y {
        x: X { x: 0 },
        p: (x.as_pointer()),
    }));
    (*y.borrow_mut()).x.x = 5;
    field!(({ YImpl::foo(&y.as_pointer(),) }), x).write(1);
    field!({ (*y.borrow()).p.clone() }, x).write(10);
    let mut p3: Ptr<Y> = (y.as_pointer());
    field!(p3.with(|__s| __s.p.clone()), x).write(100);
    field!(({ YImpl::ptr(&y.as_pointer(),) }), x).write(1);
    field!(({ YImpl::ptr(&y.as_pointer(),) }), x).write(50);
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
