extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> Ptr<i32> {
    let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
    return __tmp_0.as_pointer();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let bar: Value<i32> = Rc::new(RefCell::new((({ foo_0() }).read())));
    return 0;
}
