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
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        };
        this
    }
}
impl ByteRepr for X {}
#[derive(Default)]
pub struct Y {
    pub x: Value<X>,
    pub p: Value<Ptr<X>>,
}
impl Y {
    pub fn foo(&self) -> Ptr<X> {
        return self.x.as_pointer();
    }
    pub fn ptr(&self) -> Ptr<X> {
        return (self.x.as_pointer());
    }
}
impl Clone for Y {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Y {}
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
    (*(*({ (*y.borrow()).foo() }).upgrade().deref())
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
    (*(*({ (*y.borrow()).ptr() }).upgrade().deref())
        .x
        .borrow_mut()) = 1;
    (*(*({ (*y.borrow()).ptr() }).upgrade().deref())
        .x
        .borrow_mut()) = 50;
    return (*(*x.borrow()).x.borrow());
}
