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
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal("ABCD")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (s.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>(),
    ));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == (('A' as u8) as i32)));
    assert!(((((*bytes.borrow()).offset((1) as isize).read()) as i32) == (('B' as u8) as i32)));
    assert!(((((*bytes.borrow()).offset((2) as isize).read()) as i32) == (('C' as u8) as i32)));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == (('D' as u8) as i32)));
    assert!(((((*bytes.borrow()).offset((4) as isize).read()) as i32) == 0));
    return 0;
}
