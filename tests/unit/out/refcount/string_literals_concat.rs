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
    let joined: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::from_string_literal(
        b"alpha\nbeta\ngamma\n",
    )));
    assert!((((elem!((*joined.borrow()), 0).read()) as i32) == (('a' as i8) as i32)));
    assert!((((elem!((*joined.borrow()), 5).read()) as i32) == (('\n' as i8) as i32)));
    assert!((((elem!((*joined.borrow()), 6).read()) as i32) == (('b' as i8) as i32)));
    let arr: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"foobar\0")));
    assert!((((*arr.borrow())[(0) as usize] as i32) == (('f' as i8) as i32)));
    assert!((((*arr.borrow())[(3) as usize] as i32) == (('b' as i8) as i32)));
    assert!((((*arr.borrow())[(5) as usize] as i32) == (('r' as i8) as i32)));
    assert!((((*arr.borrow())[(6) as usize] as i32) == (('\0' as i8) as i32)));
    let split_pieces: Value<Ptr<i8>> =
        Rc::new(RefCell::new(Ptr::<i8>::from_string_literal(b"abcdefghi")));
    assert!((((elem!((*split_pieces.borrow()), 0).read()) as i32) == (('a' as i8) as i32)));
    assert!((((elem!((*split_pieces.borrow()), 3).read()) as i32) == (('d' as i8) as i32)));
    assert!((((elem!((*split_pieces.borrow()), 6).read()) as i32) == (('g' as i8) as i32)));
    assert!((((elem!((*split_pieces.borrow()), 8).read()) as i32) == (('i' as i8) as i32)));
    assert!((((elem!((*split_pieces.borrow()), 9).read()) as i32) == (('\0' as i8) as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
