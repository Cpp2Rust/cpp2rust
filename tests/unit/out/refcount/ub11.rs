extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let element: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc(10_i32)));
    let ptr: Value<Pointer<i32>> =
        Rc::new(RefCell::new((*element.borrow()).offset(1_i32 as isize)));
    let out: Value<i32> = Rc::new(RefCell::new(
        (*(*ptr.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow()),
    ));
    (*element.borrow()).delete();
    return Rc::new(RefCell::new((*out.borrow())));
}
