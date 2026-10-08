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
    let mut empty_buf: [i8; 256] = [0i8; 256];
    assert!(((empty_buf[(0) as usize] as i32) == (('\0' as i8) as i32)));
    assert!(((empty_buf[(255) as usize] as i32) == (('\0' as i8) as i32)));
    let mut prefix_buf: [i8; 32] =
        b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".map(i8::from_byte);
    assert!(((prefix_buf[(0) as usize] as i32) == (('%' as i8) as i32)));
    assert!(((prefix_buf[(1) as usize] as i32) == (('\0' as i8) as i32)));
    assert!(((prefix_buf[(31) as usize] as i32) == (('\0' as i8) as i32)));
    let mut short_buf: [i8; 16] = b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0".map(i8::from_byte);
    assert!(((short_buf[(0) as usize] as i32) == (('h' as i8) as i32)));
    assert!(((short_buf[(1) as usize] as i32) == (('i' as i8) as i32)));
    assert!(((short_buf[(2) as usize] as i32) == (('\0' as i8) as i32)));
    assert!(((short_buf[(15) as usize] as i32) == (('\0' as i8) as i32)));
    let mut exact_buf: [i8; 6] = b"hello\0".map(i8::from_byte);
    assert!(((exact_buf[(0) as usize] as i32) == (('h' as i8) as i32)));
    assert!(((exact_buf[(4) as usize] as i32) == (('o' as i8) as i32)));
    assert!(((exact_buf[(5) as usize] as i32) == (('\0' as i8) as i32)));
    assert!((::std::mem::size_of::<[i8; 6]>() == 6_usize));
    assert!(((::std::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1_usize) == 5_usize));
    assert!((::std::mem::size_of::<[i8; 1]>() == 1_usize));
    assert!(((::std::mem::size_of::<[i8; 16]>() as usize).wrapping_sub(1_usize) == 15_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
