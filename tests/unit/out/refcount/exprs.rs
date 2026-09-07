extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct X {
    pub x: Value<i32>,
}
impl Clone for X {
    fn clone(&self) -> Self {
        let __this: Value<X> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<X> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for X {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Y {
    pub x: Value<X>,
    pub p: Value<Ptr<X>>,
}
pub trait YImpl {
    fn foo(&self) -> Ptr<X>;
    fn ptr(&self) -> Ptr<X>;
}
impl Clone for Y {
    fn clone(&self) -> Self {
        let __this: Value<Y> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        }));
        let this: Ptr<Y> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Y {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.p.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<X>::from_bytes(&buf[0..4]))),
            p: Rc::new(RefCell::new(<Ptr<X>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
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
    let x: Value<X> = Rc::new(RefCell::new(X {
        x: Rc::new(RefCell::new(1)),
    }));
    let y: Value<Y> = Rc::new(RefCell::new(Y {
        x: Rc::new(RefCell::new(X {
            x: Rc::new(RefCell::new(0)),
        })),
        p: Rc::new(RefCell::new((x.as_pointer()))),
    }));
    (*(*(*y.borrow()).x.borrow()).x.borrow_mut()) = 5;
    (*(*({ YImpl::foo(&y.as_pointer()) }).upgrade().deref())
        .x
        .borrow_mut()) = 1;
    (*(*(*(*y.borrow()).p.borrow()).upgrade().deref())
        .x
        .borrow_mut()) = 10;
    let p3: Value<Ptr<Y>> = Rc::new(RefCell::new((y.as_pointer())));
    (*(*(*(*(*p3.borrow()).upgrade().deref()).p.borrow())
        .upgrade()
        .deref())
    .x
    .borrow_mut()) = 100;
    (*(*({ YImpl::ptr(&y.as_pointer()) }).upgrade().deref())
        .x
        .borrow_mut()) = 1;
    (*(*({ YImpl::ptr(&y.as_pointer()) }).upgrade().deref())
        .x
        .borrow_mut()) = 50;
    assert!(((*(*x.borrow()).x.borrow()) == 100));
    return 0;
}
impl YImpl for Ptr<Y> {
    fn foo(&self) -> Ptr<X> {
        return (*(*self).upgrade().deref()).x.as_pointer();
    }
    fn ptr(&self) -> Ptr<X> {
        return ((*(*self).upgrade().deref()).x.as_pointer());
    }
}
