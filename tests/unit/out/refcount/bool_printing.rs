extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> bool {
    return true;
}
pub fn bar_1() -> bool {
    return true;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(0));
    let i2: Value<i32> = Rc::new(RefCell::new(1));
    write!(libcc2rs::cout(), "{:}\n", (true as u8),);
    write!(libcc2rs::cout(), "{:}\n", (false as u8),);
    write!(
        libcc2rs::cout(),
        "{:}\n",
        ((((*i1.borrow()) != (*i2.borrow())) as bool) as u8),
    );
    write!(
        libcc2rs::cout(),
        "{:}\n",
        ((((*i1.borrow()) == (*i2.borrow())) as bool) as u8),
    );
    write!(libcc2rs::cout(), "{:}\n", (({ foo_0() }) as u8),);
    write!(libcc2rs::cout(), "{:}\n", (({ bar_1() }) as u8),);
    return 0;
}
