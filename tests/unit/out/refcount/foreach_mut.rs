extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v1.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v1.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v1.borrow_mut()).push(__a1)
    };
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        let x: Value<i32> = Rc::new(RefCell::new(x.read()));
        (*sum.borrow_mut()) += (*x.borrow_mut()).prefix_inc();
    }
    'loop_: for x in v1.as_pointer() as Ptr<i32> {
        let x: Value<i32> = Rc::new(RefCell::new(x.read()));
        (*sum.borrow_mut()) += (*x.borrow());
    }
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        {
            let _ptr = x.clone();
            _ptr.write(_ptr.read() + 10)
        };
    }
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        (*sum.borrow_mut()) += { (x.read()) };
    }
    let v2: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = ((v1.as_pointer() as Ptr<i32>).offset(0_usize));
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = ((v1.as_pointer() as Ptr<i32>).offset(1_usize));
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = ((v1.as_pointer() as Ptr<i32>).offset(2_usize));
        (*v2.borrow_mut()).push(__a1)
    };
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read()));
        {
            let _ptr = (*p.borrow()).clone();
            _ptr.write(_ptr.read() + 5)
        };
    }
    'loop_: for p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read()));
        (*sum.borrow_mut()) += { ((*p.borrow()).read()) };
    }
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read()));
        {
            let _ptr = (*p.borrow()).clone();
            _ptr.write(_ptr.read() + 5)
        };
    }
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read()));
        (*sum.borrow_mut()) += { ((*p.borrow()).read()) };
    }
    assert!(((*sum.borrow()) == 168));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
