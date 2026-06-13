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
    let mut joined: *const ::core::ffi::c_char = c"alpha\nbeta\ngamma\n".as_ptr();
    assert!((((*joined.offset((0) as isize)) as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!((((*joined.offset((5) as isize)) as i32) == (('\n' as ::core::ffi::c_char) as i32)));
    assert!((((*joined.offset((6) as isize)) as i32) == (('b' as ::core::ffi::c_char) as i32)));
    let mut arr: [::core::ffi::c_char; 7] = libcc2rs::char_array(b"foobar\0");
    assert!(((arr[(0) as usize] as i32) == (('f' as ::core::ffi::c_char) as i32)));
    assert!(((arr[(3) as usize] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((arr[(5) as usize] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(((arr[(6) as usize] as i32) == (('\0' as ::core::ffi::c_char) as i32)));
    let mut split_pieces: *const ::core::ffi::c_char = c"abcdefghi".as_ptr();
    assert!(
        (((*split_pieces.offset((0) as isize)) as i32) == (('a' as ::core::ffi::c_char) as i32))
    );
    assert!(
        (((*split_pieces.offset((3) as isize)) as i32) == (('d' as ::core::ffi::c_char) as i32))
    );
    assert!(
        (((*split_pieces.offset((6) as isize)) as i32) == (('g' as ::core::ffi::c_char) as i32))
    );
    assert!(
        (((*split_pieces.offset((8) as isize)) as i32) == (('i' as ::core::ffi::c_char) as i32))
    );
    assert!(
        (((*split_pieces.offset((9) as isize)) as i32) == (('\0' as ::core::ffi::c_char) as i32))
    );
    return 0;
}
