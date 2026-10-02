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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"foobar\0")));
    assert!((((((*arr.borrow())[(0) as usize] as i32) == ('f' as i32)) as i32) != 0));
    assert!((((((*arr.borrow())[(3) as usize] as i32) == ('b' as i32)) as i32) != 0));
    assert!((((((*arr.borrow())[(5) as usize] as i32) == ('r' as i32)) as i32) != 0));
    assert!((((((*arr.borrow())[(6) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let split_pieces: Value<Ptr<u8>> =
        Rc::new(RefCell::new(Ptr::<u8>::from_string_literal(b"abcdefghi")));
    assert!((((((elem!((*split_pieces.borrow()), 0).read()) as i32) == ('a' as i32)) as i32) != 0));
    assert!((((((elem!((*split_pieces.borrow()), 3).read()) as i32) == ('d' as i32)) as i32) != 0));
    assert!((((((elem!((*split_pieces.borrow()), 6).read()) as i32) == ('g' as i32)) as i32) != 0));
    assert!((((((elem!((*split_pieces.borrow()), 8).read()) as i32) == ('i' as i32)) as i32) != 0));
    assert!(
        (((((elem!((*split_pieces.borrow()), 9).read()) as i32) == ('\0' as i32)) as i32) != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
