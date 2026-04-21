extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let arr: Value<Box<[Value<i32>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(1_i32)),
        Rc::new(RefCell::new(2_i32)),
        Rc::new(RefCell::new(3_i32)),
    ])));
    let p: Value<Pointer<i32>> = Rc::new(RefCell::new(arr.as_pointer().clone()));
    (*p.borrow()).delete_array();
    return Rc::new(RefCell::new(0_i32));
}
