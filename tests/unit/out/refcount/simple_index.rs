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
    let v: Value<Vec<bool>> = Rc::new(RefCell::new(vec![true]));
    return ((*(v.as_pointer() as Ptr<bool>)
        .offset(0_u64 as isize)
        .upgrade()
        .deref()) as i32);
}
