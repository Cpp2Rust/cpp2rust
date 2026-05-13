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
    let arr: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([67305985_u32, 134678021_u32])));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u32>).reinterpret_cast::<u8>(),
    ));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == 4));
    assert!(((((*bytes.borrow()).offset((4) as isize).read()) as i32) == 5));
    (*bytes.borrow()).offset((3) as isize).write(255_u8);
    assert!(((*arr.borrow())[(0) as usize] == 4278387201));
    (*bytes.borrow()).offset((4) as isize).write(170_u8);
    assert!(((*arr.borrow())[(1) as usize] == 134678186_u32));
    return 0;
}
