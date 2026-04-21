extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn dangling() -> Reference<i32> {
    let x: Value<i32> = Rc::new(RefCell::new(1_i32));
    let p: Value<Pointer<i32>> = Rc::new(RefCell::new(x.as_pointer()));
    return Rc::downgrade(
        &(*p.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer"),
    );
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let x: Reference<i32> = dangling().clone();
    return Rc::new(RefCell::new(
        (*x.upgrade().expect("ub: dangling reference").borrow()),
    ));
}
