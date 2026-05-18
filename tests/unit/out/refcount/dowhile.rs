extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn dowhile_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'loop_: loop {
        (*x.borrow_mut()) += 1;
        'loop_: loop {
            (*x.borrow_mut()) += 1;
            (*x.borrow_mut()) += 1;
            if !((*x.borrow()) <= 100) {
                break;
            }
        }
        (*x.borrow_mut()) += 1;
        if !((*x.borrow()) <= 200) {
            break;
        }
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ({
        let _x: i32 = 0;
        dowhile_0(_x)
    });
}
