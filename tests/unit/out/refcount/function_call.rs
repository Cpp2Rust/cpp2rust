extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn function_0(y: i32, z: i32) -> i32 {
    let y: Value<i32> = Rc::new(RefCell::new(y));
    let z: Value<i32> = Rc::new(RefCell::new(z));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    return (((*x.borrow()) + (*y.borrow())) + (*z.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let y: Value<i32> = Rc::new(RefCell::new(
        ({
            let _y: i32 = 10;
            let _z: i32 = 1;
            function_0(_y, _z)
        }),
    ));
    return (*y.borrow());
}
