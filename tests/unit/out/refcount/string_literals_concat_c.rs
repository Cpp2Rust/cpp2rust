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
    let mut arr: [i8; 7] = b"foobar\0".map(i8::from_byte);
    assert!(((((arr[(0) as usize] as i32) == ('f' as i32)) as i32) != 0));
    assert!(((((arr[(3) as usize] as i32) == ('b' as i32)) as i32) != 0));
    assert!(((((arr[(5) as usize] as i32) == ('r' as i32)) as i32) != 0));
    assert!(((((arr[(6) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let mut split_pieces: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcdefghi");
    assert!((((((elem!(split_pieces, 0).read()) as i32) == ('a' as i32)) as i32) != 0));
    assert!((((((elem!(split_pieces, 3).read()) as i32) == ('d' as i32)) as i32) != 0));
    assert!((((((elem!(split_pieces, 6).read()) as i32) == ('g' as i32)) as i32) != 0));
    assert!((((((elem!(split_pieces, 8).read()) as i32) == ('i' as i32)) as i32) != 0));
    assert!((((((elem!(split_pieces, 9).read()) as i32) == ('\0' as i32)) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
