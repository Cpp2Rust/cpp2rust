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
    let N: Value<i32> = Rc::new(RefCell::new(10000));
    let arr: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*N.borrow()) as u64))
            .map(|_| <u32>::default())
            .collect::<Box<[u32]>>(),
    )));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (*arr.borrow()).offset((*i.borrow()) as isize).write(0_u32);
        (*i.borrow_mut()).postfix_inc();
    }
    (*arr.borrow())
        .offset(((*N.borrow()) - 1) as isize)
        .write(3148519816);
    let words: Value<Ptr<u16>> = Rc::new(RefCell::new(
        ((*arr.borrow()).reinterpret_cast::<u16>()).clone(),
    ));
    assert!(
        ((((*words.borrow())
            .offset((((*N.borrow()) * 2) - 1) as isize)
            .read()) as i32)
            == 48042)
    );
    assert!(
        ((((*words.borrow())
            .offset((((*N.borrow()) * 2) - 2) as isize)
            .read()) as i32)
            == 39304)
    );
    (*arr.borrow()).delete_array();
    return 0;
}
