extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(p: Ptr<i32>) -> Ptr<i32> {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).offset((5) as isize));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_u64)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < 10_u32) {
        let __rhs = ((*i.borrow()) as i32);
        (*p1.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let out: Value<i32> = Rc::new(RefCell::new(
        (({
            let _p: Ptr<i32> = ((*p1.borrow()).offset((1) as isize));
            foo_0(_p)
        })
        .offset((3) as isize)
        .read()),
    ));
    (*p1.borrow()).delete_array();
    return (*out.borrow());
}
