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
pub fn func_0() -> i32 {
    return 42;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = <Value<i32>>::default();
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    let g: Ptr<i32> = x.as_pointer();
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let z: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    let xx: Value<X> = Rc::new(RefCell::new(<X>::default()));
    let zz: Value<Ptr<X>> = Rc::new(RefCell::new((xx.as_pointer())));
    (*(*xx.borrow()).x.borrow_mut()) = 1;
    (*q.borrow_mut()) = ((*xx.borrow()).x.as_pointer());
    (*q.borrow_mut()) = ((*(*zz.borrow()).upgrade().deref()).x.as_pointer());
    (*(*(*zz.borrow()).upgrade().deref()).x.borrow_mut()) = 2;
    let ww: Value<X> = Rc::new(RefCell::new((*xx.borrow()).clone()));
    (*ww.borrow_mut()) = (*xx.borrow()).clone();
    let aa: Value<i32> = Rc::new(RefCell::new(({ func_0() })));
    (*aa.borrow_mut()) = ({ func_0() });
    return 3;
}
