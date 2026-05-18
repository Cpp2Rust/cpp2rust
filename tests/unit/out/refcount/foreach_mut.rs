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
        let rhs_0 = (((x.read()) as i32) + 10) as i32;
        x.write(rhs_0);
    }
    'loop_: for mut x in v1.as_pointer() as Ptr<i32> {
        let __rhs = (x.read());
        (*sum.borrow_mut()) += __rhs;
    }
    let v2: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(Vec::new()));
    (*v2.borrow_mut()).push(((v1.as_pointer() as Ptr<i32>).offset(0_u64 as isize)));
    (*v2.borrow_mut()).push(((v1.as_pointer() as Ptr<i32>).offset(1_u64 as isize)));
    (*v2.borrow_mut()).push(((v1.as_pointer() as Ptr<i32>).offset(2_u64 as isize)));
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        {
            let __ptr = (*p.borrow()).clone();
            let __tmp = __ptr.read() + 5;
            __ptr.write(__tmp)
        };
    }
    'loop_: for p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        let __rhs = ((*p.borrow()).read());
        (*sum.borrow_mut()) += __rhs;
    }
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        let rhs_0 = ((((*p.borrow()).read()) as i32) + 5) as i32;
        (*p.borrow()).write(rhs_0);
    }
    'loop_: for mut p in v2.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        let __rhs = ((*p.borrow()).read());
        (*sum.borrow_mut()) += __rhs;
    }
    return (*sum.borrow());
}
