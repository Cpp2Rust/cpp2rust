extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn foo(single: Value<Pointer<i32>>) {
    (*single.borrow()).delete();
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let x: Value<Pointer<i32>> = Rc::new(RefCell::new(Pointer::alloc_array(
        (0..(10_i32 as u64))
            .map(|_| <Value<i32>>::default())
            .collect::<Box<[Value<i32>]>>(),
    )));
    foo(Rc::new(RefCell::new((*x.borrow()).clone())));
    return Rc::new(RefCell::new(0_i32));
}
