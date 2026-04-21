extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn null() -> Value<Pointer<i32>> {
    let p: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::null()));
    return Rc::new(RefCell::new((*p.borrow()).clone()));
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let x: Value<Pointer<i32>> = Rc::new(RefCell::new((*null().borrow()).clone()));
    return Rc::new(RefCell::new(
        (*(*x.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow()),
    ));
}
