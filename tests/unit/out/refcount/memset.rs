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
    let N: Value<i32> = Rc::new(RefCell::new(3));
    let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*N.borrow()) as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    {
        ((*arr.borrow()).clone() as Ptr<i32>).to_any().memset(
            (1) as u8,
            (::std::mem::size_of::<i32>() as u64 as u64).wrapping_mul(((*N.borrow()) as u64))
                as usize,
        );
        ((*arr.borrow()).clone() as Ptr<i32>).to_any().clone()
    };
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = ((*arr.borrow()).offset((*i.borrow()) as isize).read());
        (*sum.borrow_mut()) += __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    (*arr.borrow()).delete_array();
    return (*sum.borrow());
}
