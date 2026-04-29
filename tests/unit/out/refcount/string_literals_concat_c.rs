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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::<[u8]>::from(b"foobar\0".as_slice())));
    assert!((((*arr.borrow())[(0) as usize] as i32) == ('f' as i32)));
    assert!((((*arr.borrow())[(3) as usize] as i32) == ('b' as i32)));
    assert!((((*arr.borrow())[(5) as usize] as i32) == ('r' as i32)));
    assert!((((*arr.borrow())[(6) as usize] as i32) == ('\0' as i32)));
    let split_pieces: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("abcdefghi")));
    assert!(((((*split_pieces.borrow()).offset((0) as isize).read()) as i32) == ('a' as i32)));
    assert!(((((*split_pieces.borrow()).offset((3) as isize).read()) as i32) == ('d' as i32)));
    assert!(((((*split_pieces.borrow()).offset((6) as isize).read()) as i32) == ('g' as i32)));
    assert!(((((*split_pieces.borrow()).offset((8) as isize).read()) as i32) == ('i' as i32)));
    assert!(((((*split_pieces.borrow()).offset((9) as isize).read()) as i32) == ('\0' as i32)));
    return 0;
}
