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
    let array: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..100_u64)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    {
        ((*array.borrow()).clone() as Ptr<i32>).to_any().memset(
            (0) as u8,
            (::std::mem::size_of::<i32>() as u64 as u64).wrapping_mul(100_u64) as usize,
        );
        ((*array.borrow()).clone() as Ptr<i32>).to_any().clone()
    };
    (*array.borrow()).offset((99) as isize).write(-1_i32);
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((*array.borrow()).clone()));
    'loop_: while (((*p1.borrow()).read()) >= 0) {
        (*p1.borrow()).write(1);
        (*p1.borrow_mut()).prefix_inc();
    }
    let out: Value<i32> = Rc::new(RefCell::new(0));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((*array.borrow()).clone()));
    'loop_: while (((*p1.borrow()).read()) >= 0) {
        let __rhs = ((*p1.borrow()).read());
        (*out.borrow_mut()) += __rhs;
        (*p1.borrow_mut()).prefix_inc();
    }
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((*array.borrow()).clone()));
    (*p2.borrow()).delete_array();
    return (*out.borrow());
}
