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
    let mut arr: [libc::c_char; 3] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
    ];
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_u8 as i32) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_i8 as i32) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_u16 as i32) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_i16 as i32) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_u32 as u32) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add(((1 as i32) as i32) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_u64 as u64) as usize);
    assert!((((*p) as i32) == (2)));
    let mut p: *mut libc::c_char = (&mut arr[(0) as usize] as *mut libc::c_char);
    p = (p).wrapping_add((1_i64 as i64) as usize);
    assert!((((*p) as i32) == (2)));
    return 0;
}
