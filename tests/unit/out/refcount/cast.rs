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
    let size: Value<u64> = Rc::new(RefCell::new(1_u64));
    if ((*size.borrow()) == 1_u64) {
        return 1;
    }
    return 0;
}
