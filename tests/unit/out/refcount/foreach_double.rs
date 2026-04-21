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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(1);
    (*v.borrow_mut()).push(2);
    (*v.borrow_mut()).push(3);
    let square: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        let e1: Value<i32> = Rc::new(RefCell::new(e1.read().clone()));
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            let e2: Value<i32> = Rc::new(RefCell::new(e2.read().clone()));
            (*square.borrow_mut()) += ((*e1.borrow()) * (*e2.borrow()));
        }
    }
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            let __rhs = {
                let _lhs = (e1.read());
                _lhs * (e2.read())
            };
            (*square.borrow_mut()) += __rhs;
        }
    }
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            let __rhs = {
                let _lhs = (e1.read());
                _lhs * (e2.read())
            };
            (*square.borrow_mut()) += __rhs;
        }
    }
    'loop_: for mut e1 in v.as_pointer() as Ptr<i32> {
        'loop_: for mut e2 in v.as_pointer() as Ptr<i32> {
            let __rhs = {
                let _lhs = (e1.read());
                _lhs * (e2.read())
            };
            (*square.borrow_mut()) += __rhs;
        }
    }
    let m: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (m.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new((*v1.borrow_mut()).clone())))
    });
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (m.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new((*v2.borrow_mut()).clone())))
    });
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (m.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new((*v3.borrow_mut()).clone())))
    });
    'loop_: for mut row in m.as_pointer() as Ptr<Value<Vec<i32>>> {
        let row: Ptr<Vec<i32>> = row.upgrade().deref().as_pointer();
        'loop_: for mut col in row.to_strong().as_pointer() as Ptr<i32> {
            let __rhs = (col.read());
            (*square.borrow_mut()) += __rhs;
        }
    }
    return (*square.borrow());
}
