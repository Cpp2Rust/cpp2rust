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
    let mut empty_buf : [ u8 ; 256] = * b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" ;
    assert!(((empty_buf[(0) as usize] as i32) == ('\0' as i32)));
    assert!(((empty_buf[(255) as usize] as i32) == ('\0' as i32)));
    let mut prefix_buf: [u8; 32] =
        *b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    assert!(((prefix_buf[(0) as usize] as i32) == ('%' as i32)));
    assert!(((prefix_buf[(1) as usize] as i32) == ('\0' as i32)));
    assert!(((prefix_buf[(31) as usize] as i32) == ('\0' as i32)));
    let mut short_buf: [u8; 16] = *b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    assert!(((short_buf[(0) as usize] as i32) == ('h' as i32)));
    assert!(((short_buf[(1) as usize] as i32) == ('i' as i32)));
    assert!(((short_buf[(2) as usize] as i32) == ('\0' as i32)));
    assert!(((short_buf[(15) as usize] as i32) == ('\0' as i32)));
    let mut exact_buf: [u8; 6] = *b"hello\0";
    assert!(((exact_buf[(0) as usize] as i32) == ('h' as i32)));
    assert!(((exact_buf[(4) as usize] as i32) == ('o' as i32)));
    assert!(((exact_buf[(5) as usize] as i32) == ('\0' as i32)));
    assert!(((::std::mem::size_of::<[u8; 6]>() as u64) == (6_u64)));
    assert!((((::std::mem::size_of::<[u8; 6]>() as u64 as u64).wrapping_sub(1_u64)) == (5_u64)));
    assert!(((::std::mem::size_of::<[u8; 1]>() as u64) == (1_u64)));
    assert!((((::std::mem::size_of::<[u8; 16]>() as u64 as u64).wrapping_sub(1_u64)) == (15_u64)));
    return 0;
}
