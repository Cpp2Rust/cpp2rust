extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type color = u32;
pub const color_RED: color = 0;
pub const color_GREEN: color = 1;
pub const color_BLUE: color = 2;
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<color> = Rc::new(RefCell::new(color_BLUE));
    (*c.borrow_mut()).postfix_inc();
    return if (((((*c.borrow()) as u32) == ((color_RED as i32) as u32)) as i32) != 0) {
        0
    } else {
        1
    };
}
