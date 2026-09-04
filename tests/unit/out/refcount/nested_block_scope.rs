extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(1));
    {
        let x: Value<i32> = Rc::new(RefCell::new(2));
        assert!(((*x.borrow()) == 2));
        {
            let x: Value<i32> = Rc::new(RefCell::new(3));
            assert!(((*x.borrow()) == 3));
        }
        assert!(((*x.borrow()) == 2));
    }
    assert!(((*x.borrow()) == 1));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        let y: Value<i32> = Rc::new(RefCell::new((*i.borrow())));
        {
            let y: Value<i32> = Rc::new(RefCell::new(10));
            (*sum.borrow_mut()) += (*y.borrow());
        }
        (*sum.borrow_mut()) += (*y.borrow());
        (*i.borrow_mut()).postfix_inc();
    }
    assert!(((*sum.borrow()) == 33));
    if ((*x.borrow()) == 1) {
        let x: Value<i32> = Rc::new(RefCell::new(5));
        {
            let x: Value<i32> = Rc::new(RefCell::new(6));
            assert!(((*x.borrow()) == 6));
        }
        assert!(((*x.borrow()) == 5));
    }
    assert!(((*x.borrow()) == 1));
    return 0;
}
