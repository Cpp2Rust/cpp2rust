extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..2_u64).map(|_| <i32>::default()).collect::<Box<[_]>>(),
    )))));
    (*g.borrow()).as_ref().unwrap().borrow_mut()[(0_u64) as usize] = 11;
    (*g.borrow()).as_ref().unwrap().borrow_mut()[(1_u64) as usize] = 12;
    let g_ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((*g.borrow()).as_pointer()));
    (*g_ptr.borrow()).offset((0) as isize).write(13);
    (*g_ptr.borrow()).offset((1) as isize).write(14);
    return ((*g.borrow()).as_ref().unwrap().borrow()[(0_u64) as usize]
        + (*g.borrow()).as_ref().unwrap().borrow()[(1_u64) as usize]);
}
