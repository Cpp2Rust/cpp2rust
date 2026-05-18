extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(u: Option<Value<i32>>) -> Option<Value<i32>> {
    let u: Value<Option<Value<i32>>> = Rc::new(RefCell::new(u));
    (*(*u.borrow_mut()).as_ref().unwrap().borrow_mut()) = 10;
    return (*u.borrow_mut()).take();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let f: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(8)))));
    (*(*f.borrow_mut()).as_ref().unwrap().borrow_mut()) = 9;
    let f_ptr1: Value<Ptr<i32>> = Rc::new(RefCell::new((*f.borrow()).as_pointer()));
    (*f_ptr1.borrow()).write(10);
    let f_ptr2: Value<Ptr<i32>> = Rc::new(RefCell::new(((*f.borrow()).as_pointer())));
    (*f_ptr2.borrow()).write(11);
    (*f.borrow_mut()) = Some(Rc::new(RefCell::new(9)));
    let __rhs = ({
        let _u: Option<Value<i32>> = (*f.borrow_mut()).take();
        fn_0(_u)
    });
    (*f.borrow_mut()) = __rhs;
    return (*(*f.borrow()).as_ref().unwrap().borrow());
}
