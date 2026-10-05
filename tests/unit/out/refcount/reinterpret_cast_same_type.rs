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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8])));
    let mut same: Ptr<u8> = (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>();
    assert!((((elem!(same, 0).read()) as i32) == 10));
    assert!((((elem!(same, 1).read()) as i32) == 20));
    assert!((((elem!(same, 2).read()) as i32) == 30));
    assert!((((elem!(same, 3).read()) as i32) == 40));
    elem!(same, 2).write(99_u8);
    assert!((((*arr.borrow())[(2) as usize] as i32) == 99));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
