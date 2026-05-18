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
    let original: Value<i32> = Rc::new(RefCell::new(67305985));
    let halves: Value<Ptr<i16>> = Rc::new(RefCell::new(
        (original.as_pointer()).reinterpret_cast::<i16>(),
    ));
    assert!(((((*halves.borrow()).offset((0) as isize).read()) as i32) == 513));
    assert!(((((*halves.borrow()).offset((1) as isize).read()) as i32) == 1027));
    (*halves.borrow()).offset((0) as isize).write(4112_i16);
    assert!(((*original.borrow()) == 67309584));
    let arr: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([513_i16, 1027_i16])));
    let as_int: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<i16>).reinterpret_cast::<i32>(),
    ));
    assert!((((*as_int.borrow()).read()) == 67305985));
    return 0;
}
