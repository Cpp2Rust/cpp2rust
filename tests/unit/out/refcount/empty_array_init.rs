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
    let vec_: Value<[i32; 3]> = Rc::new(RefCell::new(std::array::from_fn::<_, 3, _>(|_| 0_i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
