extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn smaller(x1: Reference<i32>, x2: Reference<i32>) -> Value<Pointer<i32>> {
    return Rc::new(RefCell::new(
        if ((*x1.upgrade().expect("ub: dangling reference").borrow())
            < (*x2.upgrade().expect("ub: dangling reference").borrow()))
        {
            x1.as_pointer().clone()
        } else {
            x2.as_pointer().clone()
        },
    ));
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let out: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::null()));
    let x1: Value<i32> = Rc::new(RefCell::new(1_i32));
    if ((*x1.borrow()) != 0) {
        let x2: Value<i32> = Rc::new(RefCell::new(-1_i32));
        (*out.borrow_mut()) = (*smaller(Rc::downgrade(&x1), Rc::downgrade(&x2)).borrow()).clone();
    }
    return Rc::new(RefCell::new(
        (*(*out.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow()),
    ));
}
