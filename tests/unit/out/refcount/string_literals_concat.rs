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
    let joined: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        "alpha\nbeta\ngamma\n",
    )));
    assert!(((((*joined.borrow()).offset((0) as isize).read()) as i32) == (('a' as u8) as i32)));
    assert!(((((*joined.borrow()).offset((5) as isize).read()) as i32) == (('\n' as u8) as i32)));
    assert!(((((*joined.borrow()).offset((6) as isize).read()) as i32) == (('b' as u8) as i32)));
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::<[u8]>::from(b"foobar\0".as_slice())));
    assert!((((*arr.borrow())[(0) as usize] as i32) == (('f' as u8) as i32)));
    assert!((((*arr.borrow())[(3) as usize] as i32) == (('b' as u8) as i32)));
    assert!((((*arr.borrow())[(5) as usize] as i32) == (('r' as u8) as i32)));
    assert!((((*arr.borrow())[(6) as usize] as i32) == (('\0' as u8) as i32)));
    let split_pieces: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("abcdefghi")));
    assert!(
        ((((*split_pieces.borrow()).offset((0) as isize).read()) as i32) == (('a' as u8) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((3) as isize).read()) as i32) == (('d' as u8) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((6) as isize).read()) as i32) == (('g' as u8) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((8) as isize).read()) as i32) == (('i' as u8) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((9) as isize).read()) as i32) == (('\0' as u8) as i32))
    );
    return 0;
}
