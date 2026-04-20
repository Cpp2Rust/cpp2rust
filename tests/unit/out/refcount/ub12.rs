extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn escape(ptr: Value<Pointer<i32>>) {
    (*ptr.borrow()).delete();
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let alloc: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc(1_i32)));
    escape(Rc::new(RefCell::new((*alloc.borrow()).clone())));
    (*alloc.borrow()).delete();
    return Rc::new(RefCell::new(0_i32));
}
