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
    let vec_: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    (*vec_.borrow_mut()).push(67305985_u32);
    (*vec_.borrow_mut()).push(134678021_u32);
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (vec_.as_pointer() as Ptr<u32>).reinterpret_cast::<u8>(),
    ));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == 1));
    assert!(((((*bytes.borrow()).offset((1) as isize).read()) as i32) == 2));
    assert!(((((*bytes.borrow()).offset((2) as isize).read()) as i32) == 3));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == 4));
    assert!(((((*bytes.borrow()).offset((4) as isize).read()) as i32) == 5));
    assert!(((((*bytes.borrow()).offset((7) as isize).read()) as i32) == 8));
    (*bytes.borrow()).offset((4) as isize).write(255_u8);
    assert!(
        (((vec_.as_pointer() as Ptr<u32>)
            .offset(1_u64 as isize)
            .read())
            == 134678271_u32)
    );
    return 0;
}
