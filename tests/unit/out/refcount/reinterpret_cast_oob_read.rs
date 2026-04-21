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
    let val: Value<u32> = Rc::new(RefCell::new(67305985_u32));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((val.as_pointer()).reinterpret_cast::<u8>()));
    let x: Value<u8> = Rc::new(RefCell::new(
        ((*bytes.borrow()).offset((4) as isize).read()),
    ));
    return 0;
}
