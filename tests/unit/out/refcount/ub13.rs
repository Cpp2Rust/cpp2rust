extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn escape(p: Value<Pointer<i32>>) {
    (*p.borrow()).delete();
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let p1: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc(1_i32)));
    escape(Rc::new(RefCell::new((*p1.borrow()).clone())));
    return Rc::new(RefCell::new(
        (*(*p1.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow()),
    ));
}
