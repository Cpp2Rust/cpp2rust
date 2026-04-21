extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn null(p: Value<Pointer<Pointer<i32>>>) {
    (*(*p.borrow())
        .as_reference()
        .upgrade()
        .expect("ub: dangling pointer")
        .borrow_mut()) = Pointer::null();
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let x: Value<i32> = Rc::new(RefCell::new(1_i32));
    let p: Value<Pointer<i32>> = Rc::new(RefCell::new(x.as_pointer()));
    null(Rc::new(RefCell::new(p.as_pointer())));
    let r: Reference<i32> = Rc::downgrade(
        &(*p.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer"),
    );
    return Rc::new(RefCell::new(
        (*r.upgrade().expect("ub: dangling reference").borrow()),
    ));
}
