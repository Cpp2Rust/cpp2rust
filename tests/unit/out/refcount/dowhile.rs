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
    let mut __do_while = true;
    'loop_: while __do_while || ((*x.borrow()) <= 200) {
        __do_while = false;
        (*x.borrow_mut()) += 1;
        let mut __do_while = true;
        'loop_: while __do_while || ((*x.borrow()) <= 100) {
            __do_while = false;
            (*x.borrow_mut()) += 1;
            (*x.borrow_mut()) += 1;
        }
        (*x.borrow_mut()) += 1;
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ({ dowhile_0(0) });
}
