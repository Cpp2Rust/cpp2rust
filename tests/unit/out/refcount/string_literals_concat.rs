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
    let joined: Value<Ptr<::core::ffi::c_char>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"alpha\nbeta\ngamma\n",
    )));
    assert!(
        ((((*joined.borrow()).offset((0) as isize).read()) as i32)
            == (('a' as ::core::ffi::c_char) as i32))
    );
    assert!(
        ((((*joined.borrow()).offset((5) as isize).read()) as i32)
            == (('\n' as ::core::ffi::c_char) as i32))
    );
    assert!(
        ((((*joined.borrow()).offset((6) as isize).read()) as i32)
            == (('b' as ::core::ffi::c_char) as i32))
    );
    let arr: Value<Box<[::core::ffi::c_char]>> =
        Rc::new(RefCell::new(Box::from(libcc2rs::char_array(b"foobar\0"))));
    assert!((((*arr.borrow())[(0) as usize] as i32) == (('f' as ::core::ffi::c_char) as i32)));
    assert!((((*arr.borrow())[(3) as usize] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!((((*arr.borrow())[(5) as usize] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!((((*arr.borrow())[(6) as usize] as i32) == (('\0' as ::core::ffi::c_char) as i32)));
    let split_pieces: Value<Ptr<::core::ffi::c_char>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"abcdefghi")));
    assert!(
        ((((*split_pieces.borrow()).offset((0) as isize).read()) as i32)
            == (('a' as ::core::ffi::c_char) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((3) as isize).read()) as i32)
            == (('d' as ::core::ffi::c_char) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((6) as isize).read()) as i32)
            == (('g' as ::core::ffi::c_char) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((8) as isize).read()) as i32)
            == (('i' as ::core::ffi::c_char) as i32))
    );
    assert!(
        ((((*split_pieces.borrow()).offset((9) as isize).read()) as i32)
            == (('\0' as ::core::ffi::c_char) as i32))
    );
    return 0;
}
