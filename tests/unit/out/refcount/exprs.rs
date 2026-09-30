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
    field!(({ YImpl::foo(&y.as_pointer(),) }), x).write(1);
    field!({ (*y.borrow()).p.clone() }, x).write(10);
    let p3: Value<Ptr<Y>> = Rc::new(RefCell::new((y.as_pointer())));
    field!((*p3.borrow()).with(|__s| __s.p.clone()), x).write(100);
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
