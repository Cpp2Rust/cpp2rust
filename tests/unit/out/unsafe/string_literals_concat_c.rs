extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [u8; 7] = *b"foobar\0";
    assert!(((((arr[(0) as usize] as i32) == ('f' as i32)) as i32) != 0));
    assert!(((((arr[(3) as usize] as i32) == ('b' as i32)) as i32) != 0));
    assert!(((((arr[(5) as usize] as i32) == ('r' as i32)) as i32) != 0));
    assert!(((((arr[(6) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let mut split_pieces: *const u8 = b"abcdefghi\0".as_ptr().cast_mut().cast_const();
    assert!((((((*split_pieces.offset((0) as isize)) as i32) == ('a' as i32)) as i32) != 0));
    assert!((((((*split_pieces.offset((3) as isize)) as i32) == ('d' as i32)) as i32) != 0));
    assert!((((((*split_pieces.offset((6) as isize)) as i32) == ('g' as i32)) as i32) != 0));
    assert!((((((*split_pieces.offset((8) as isize)) as i32) == ('i' as i32)) as i32) != 0));
    assert!((((((*split_pieces.offset((9) as isize)) as i32) == ('\0' as i32)) as i32) != 0));
    return 0;
}
