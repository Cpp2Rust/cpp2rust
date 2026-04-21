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
    let cond: Value<bool> = Rc::new(RefCell::new(true));
    let os1: Value<Ptr<std::fs::File>> = Rc::new(RefCell::new(if (*cond.borrow()) {
        libcc2rs::cout()
    } else {
        libcc2rs::cerr()
    }));
    write!((*os1.borrow()), "hello\n",);
    let os2: Ptr<std::fs::File> = if (*cond.borrow()) {
        libcc2rs::cout()
    } else {
        libcc2rs::cerr()
    };
    write!(os2, "hello\n",);
    return 0;
}
