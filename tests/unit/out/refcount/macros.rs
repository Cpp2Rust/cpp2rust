extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn log_0(mut file: Ptr<i8>, mut line: i32, mut func: Ptr<i8>) {
    println!("{} {} {}", file, line, func);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!(
        "{} {} {}",
        Ptr::<i8>::from_string_literal(b"macros.cpp"),
        8,
        Ptr::<i8>::from_string_literal(b"main")
    );
    ({
        log_0(
            Ptr::<i8>::from_string_literal(b"macros.cpp"),
            9,
            Ptr::<i8>::from_string_literal(b"main"),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
