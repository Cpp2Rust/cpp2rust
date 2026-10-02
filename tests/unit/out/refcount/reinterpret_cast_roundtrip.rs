extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let original: Value<i32> = Rc::new(RefCell::new(67305985));
    let halves: Value<Ptr<i16>> = Rc::new(RefCell::new(
        (original.as_pointer()).reinterpret_cast::<i16>(),
    ));
    assert!((((elem!((*halves.borrow()), 0).read()) as i32) == 513));
    assert!((((elem!((*halves.borrow()), 1).read()) as i32) == 1027));
    elem!((*halves.borrow()), 0).write(4112_i16);
    assert!(((*original.borrow()) == 67309584));
    let arr: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([513_i16, 1027_i16])));
    let as_int: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<i16>).reinterpret_cast::<i32>(),
    ));
    assert!((((*as_int.borrow()).read()) == 67305985));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
