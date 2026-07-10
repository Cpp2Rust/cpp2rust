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
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v1.borrow_mut()).push(1);
    (*v1.borrow_mut()).push(2);
    (*v1.borrow_mut()).push(3);
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        let x: Value<i32> = Rc::new(RefCell::new(x.read().clone()));
        (*sum.borrow_mut()) += (*x.borrow_mut()).prefix_inc();
    }
    'loop_: for x in v1.as_pointer() as Ptr<i32> {
        let x: Value<i32> = Rc::new(RefCell::new(x.read().clone()));
        (*sum.borrow_mut()) += (*x.borrow());
    }
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        {
            let _ptr = x.clone();
            _ptr.write(_ptr.read() + 10)
        };
    }
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        let __rhs = (x.read());
        (*sum.borrow_mut()) += __rhs;
    }
    let v2: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(Vec::new()));
    (*v2.borrow_mut()).push(((v1.as_pointer() as Ptr<i32>).offset(0_usize)));
    (*v2.borrow_mut()).push(((v1.as_pointer() as Ptr<i32>).offset(1_usize)));
    (*v2.borrow_mut()).push(((v1.as_pointer() as Ptr<i32>).offset(2_usize)));
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        {
            let _ptr = (*p.borrow()).clone();
            _ptr.write(_ptr.read() + 5)
        };
    }
    'loop_: for p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        let __rhs = ((*p.borrow()).read());
        (*sum.borrow_mut()) += __rhs;
    }
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        {
            let _ptr = (*p.borrow()).clone();
            _ptr.write(_ptr.read() + 5)
        };
    }
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        let __rhs = ((*p.borrow()).read());
        (*sum.borrow_mut()) += __rhs;
    }
    return (*sum.borrow());
}
