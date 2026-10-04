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
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"ABCD").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (s.as_pointer() as Ptr<i8>).reinterpret_cast::<u8>(),
    ));
    assert!((((elem!((*bytes.borrow()), 0).read()) as i32) == (('A' as i8) as i32)));
    assert!((((elem!((*bytes.borrow()), 1).read()) as i32) == (('B' as i8) as i32)));
    assert!((((elem!((*bytes.borrow()), 2).read()) as i32) == (('C' as i8) as i32)));
    assert!((((elem!((*bytes.borrow()), 3).read()) as i32) == (('D' as i8) as i32)));
    assert!((((elem!((*bytes.borrow()), 4).read()) as i32) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
