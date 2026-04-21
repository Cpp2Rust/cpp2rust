extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let arr: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc_array(
        (0..(10_i32 as u64))
            .map(|_| <Value<i32>>::default())
            .collect::<Box<[Value<i32>]>>(),
    )));
    let ptr: Value<Pointer<i32>> = Rc::new(RefCell::new((*arr.borrow()).offset(1_i32 as isize)));
    let out: Value<i32> = Rc::new(RefCell::new(
        (*(*ptr.borrow())
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow()),
    ));
    (*ptr.borrow()).delete_array();
    return Rc::new(RefCell::new((*out.borrow())));
}
