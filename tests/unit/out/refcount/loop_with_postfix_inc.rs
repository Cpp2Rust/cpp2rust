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
    let x: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while (((*x.borrow_mut()).postfix_inc() < 100) && ((*x.borrow()) != 50)) {
        (*x.borrow_mut()).prefix_inc();
    }
    return (*x.borrow());
}
