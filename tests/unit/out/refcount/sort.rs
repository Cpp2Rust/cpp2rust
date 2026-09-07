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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(10);
    (*v.borrow_mut()).push(1);
    (*v.borrow_mut()).push(9);
    (*v.borrow_mut()).push(2);
    (*v.borrow_mut()).push(8);
    (*v.borrow_mut()).push(3);
    (*v.borrow_mut()).push(7);
    (*v.borrow_mut()).push(4);
    (*v.borrow_mut()).push(5);
    (*v.borrow_mut()).push(6);
    (v.as_pointer() as Ptr<i32>).sort((v.as_pointer() as Ptr<i32>).to_end().get_offset());
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < ((*v.borrow()).len()).wrapping_sub(1_usize)) {
        assert!(
            (((v.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as usize))
                .read())
                < ((v.as_pointer() as Ptr<i32>)
                    .offset((((*i.borrow()).wrapping_add(1_u32)) as usize))
                    .read()))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    return 0;
}
