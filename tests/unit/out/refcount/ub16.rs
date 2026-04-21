extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn foo(a: Value<Pointer<i32>>) -> Value<Pointer<i32>> {
    return Rc::new(RefCell::new((*a.borrow()).offset(5_i32 as isize)));
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let p1: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc_array(
        (0..(10_i32 as u64))
            .map(|_| <Value<i32>>::default())
            .collect::<Box<[Value<i32>]>>(),
    )));
    let out: Value<i32> = Rc::new(RefCell::new(
        (*(*foo(Rc::new(RefCell::new((*p1.borrow()).offset(1_i32 as isize)))).borrow())
            .clone()
            .offset(4_i32 as isize)
            .as_reference()
            .upgrade()
            .expect("ub: dangling pointer")
            .borrow()),
    ));
    (*p1.borrow()).delete_array();
    return Rc::new(RefCell::new(0_i32));
}
