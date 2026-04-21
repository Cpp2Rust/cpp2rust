extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn strlen(s: Value<Pointer<i8>>) -> Value<u64> {
    let begin: Value<Pointer<i8>> = Rc::new(RefCell::new((*s.borrow()).clone()));
    while ((*(*s.borrow())
        .as_reference()
        .upgrade()
        .expect("ub: dangling pointer")
        .borrow())
        != 0)
    {
        (*s.borrow_mut()).prefix_inc();
    }
    return Rc::new(RefCell::new(
        ((*s.borrow()).clone() - (*begin.borrow()).clone()) as u64,
    ));
}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let s: Value<Box<[Value<i8>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(('s' as i8))),
        Rc::new(RefCell::new(('t' as i8))),
        Rc::new(RefCell::new(('r' as i8))),
        Rc::new(RefCell::new(('i' as i8))),
        Rc::new(RefCell::new(('n' as i8))),
        Rc::new(RefCell::new(('g' as i8))),
    ])));
    return Rc::new(RefCell::new(
        ((*strlen(Rc::new(RefCell::new(s.as_pointer().offset(0_i32 as isize)))).borrow()) as i32),
    ));
}
