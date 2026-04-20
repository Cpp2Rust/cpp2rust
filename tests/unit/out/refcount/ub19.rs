extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn foo(array: Value<Pointer<i32>>) {
    (*array.borrow()).delete_array();
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let x: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc(1_i32)));
    foo(Rc::new(RefCell::new((*x.borrow()).clone())));
    return Rc::new(RefCell::new(0_i32));
}
