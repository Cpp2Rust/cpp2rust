extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(p: Ptr<i32>, n: i32) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        let __rhs = ((*p.borrow()).offset((*i.borrow()) as isize).read());
        (*total.borrow_mut()) += __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*total.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let array: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..100_usize)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    (*array.borrow()).delete();
    let filled: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..4_usize)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 4) {
        let __rhs = ((*i.borrow()) + 1);
        (*filled.borrow())
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    if (({
        let _p: Ptr<i32> = (*filled.borrow()).clone();
        sum_0(_p, 4)
    }) != 10)
    {
        return 1;
    }
    (*filled.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
