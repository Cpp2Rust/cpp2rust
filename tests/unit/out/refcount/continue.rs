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
    let out: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        if (((*i.borrow()) % 2) == 0) {
            (*i.borrow_mut()).prefix_inc();
            continue 'loop_;
        }
        (*out.borrow_mut()).prefix_inc();
        (*i.borrow_mut()).prefix_inc();
    }
    let j: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*j.borrow()) < 10) {
        if (((*i.borrow()) % 2) == 0) {
            (*j.borrow_mut()).postfix_inc();
            continue 'loop_;
        }
        (*out.borrow_mut()).prefix_inc();
        (*j.borrow_mut()).postfix_inc();
    }
    let k1: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k1.borrow()) < 5) {
        let k2: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*k2.borrow()) < 5) {
            let k3: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*k3.borrow()) < 5) {
                if ((((((*k1.borrow()) + (*k2.borrow())) + (*k3.borrow())) as i32) % 2) == 0) {
                    (*k3.borrow_mut()).postfix_inc();
                    continue 'loop_;
                }
                (*out.borrow_mut()).prefix_inc();
                (*k3.borrow_mut()).postfix_inc();
            }
            if (((((*k1.borrow()) + (*k2.borrow())) as i32) % 2) == 0) {
                (*k2.borrow_mut()).postfix_inc();
                continue 'loop_;
            }
            (*out.borrow_mut()).prefix_inc();
            (*k2.borrow_mut()).postfix_inc();
        }
        if (((*k1.borrow()) % 2) == 0) {
            (*k1.borrow_mut()).postfix_inc();
            continue 'loop_;
        }
        (*out.borrow_mut()).prefix_inc();
        (*k1.borrow_mut()).postfix_inc();
    }
    return (*out.borrow());
}
