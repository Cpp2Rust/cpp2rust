extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn log_0(file: Ptr<u8>, line: i32, func: Ptr<u8>) {
    let file: Value<Ptr<u8>> = Rc::new(RefCell::new(file));
    let line: Value<i32> = Rc::new(RefCell::new(line));
    let func: Value<Ptr<u8>> = Rc::new(RefCell::new(func));
    println!(
        "{} {} {}",
        (*file.borrow()),
        (*line.borrow()),
        (*func.borrow())
    );
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!(
        "{} {} {}",
        Ptr::from_string_literal(b"macros.cpp"),
        8,
        Ptr::from_string_literal(b"main")
    );
    ({
        let _func: Ptr<u8> = Ptr::from_string_literal(b"main");
        log_0(Ptr::from_string_literal(b"macros.cpp"), 9, _func)
    });
    return 0;
}
