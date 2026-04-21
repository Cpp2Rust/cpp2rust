extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct XX {
    pub x: Value<i32>,
}
impl Clone for XX {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        };
        this
    }
}
impl ByteRepr for XX {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let obj: Value<XX> = Rc::new(RefCell::new(<XX>::default()));
    let ptr: Value<Ptr<XX>> = Rc::new(RefCell::new((obj.as_pointer())));
    (*(*(*ptr.borrow()).upgrade().deref()).x.borrow_mut()) = 2;
    let c: Value<bool> = Rc::new(RefCell::new(false));
    let r: Value<i32> = Rc::new(RefCell::new(if (*c.borrow()) {
        (*(*obj.borrow()).x.borrow())
    } else {
        (*(*(*ptr.borrow()).upgrade().deref()).x.borrow())
    }));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(((*obj.borrow()).x.as_pointer())));
    return {
        let _lhs = ((*p.borrow()).read());
        _lhs + (*r.borrow())
    };
}
