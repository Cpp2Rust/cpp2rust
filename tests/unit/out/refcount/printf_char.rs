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
    let mut c: i8 = (('a' as i32) as i8);
    let mut n: i32 = 3;
    println!("{}", (c as i32) as u8 as char);
    println!("{} {}", n, (c as i32) as u8 as char);
    println!("100% {}", (c as i32) as u8 as char);
    println!(
        "{}{}{}",
        (c as i32) as u8 as char,
        ((c as i32) + 1) as u8 as char,
        n
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
