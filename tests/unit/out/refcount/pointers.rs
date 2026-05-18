extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Test {
    pub x: Value<i32>,
}
impl Test {
    pub fn inc(&self) {
        (*self.x.borrow_mut()).postfix_inc();
    }
    pub fn dec(&self) {
        (*self.x.borrow_mut()).postfix_dec();
    }
    pub fn as_ptr(&self) -> Ptr<i32> {
        return (self.x.as_pointer());
    }
    pub fn update(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        (*self.x.borrow_mut()) = ((*x.borrow()) + (*y.borrow()));
    }
}
impl Clone for Test {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        };
        this
    }
}
impl ByteRepr for Test {}
pub fn Update_0(t: Ptr<Test>) -> Ptr<Test> {
    let t: Value<Ptr<Test>> = Rc::new(RefCell::new(t));
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    (*x.borrow_mut()).prefix_inc();
    ({
        let _x: i32 = (*x.borrow());
        let _y: i32 = (*y.borrow());
        (*(*t.borrow()).upgrade().deref()).update(_x, _y)
    });
    (*x.borrow_mut()) = (*(*(*t.borrow()).upgrade().deref()).x.borrow());
    (*y.borrow_mut()) = (*(*(*t.borrow()).upgrade().deref()).x.borrow());
    ({
        let _x: i32 = (*x.borrow());
        let _y: i32 = (*y.borrow());
        (*(*t.borrow()).upgrade().deref()).update(_x, _y)
    });
    return (*t.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t1: Value<Test> = Rc::new(RefCell::new(Test {
        x: Rc::new(RefCell::new(100)),
    }));
    let t2: Value<Ptr<Test>> = Rc::new(RefCell::new(
        ({
            let _t: Ptr<Test> = (t1.as_pointer());
            Update_0(_t)
        }),
    ));
    let t3: Value<Ptr<Test>> = Rc::new(RefCell::new(Ptr::<Test>::null()));
    (*t3.borrow_mut()) = (*t2.borrow()).clone();
    (*(*(*t3.borrow()).upgrade().deref()).x.borrow_mut()) = 15;
    {
        let __ptr = ({ (*(*t3.borrow()).upgrade().deref()).as_ptr() }).clone();
        let __tmp = __ptr.read() + 10;
        __ptr.write(__tmp)
    };
    return {
        let _lhs = {
            let _lhs = (*(*(*t3.borrow()).upgrade().deref()).x.borrow());
            _lhs + (*(*(*t2.borrow()).upgrade().deref()).x.borrow())
        };
        _lhs + (*(*t1.borrow()).x.borrow())
    };
}
