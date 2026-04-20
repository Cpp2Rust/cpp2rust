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
    let mut g: Option<Box<[i32]>> =
        Some((0..2_u64).map(|_| <i32>::default()).collect::<Box<[_]>>());
    g.as_mut().unwrap()[(0_u64) as usize] = 11;
    g.as_mut().unwrap()[(1_u64) as usize] = 12;
    let mut g_ptr: *mut i32 = g
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr());
    (*g_ptr.offset((0) as isize)) = 13;
    (*g_ptr.offset((1) as isize)) = 14;
    return ((g.as_mut().unwrap()[(0_u64) as usize]) + (g.as_mut().unwrap()[(1_u64) as usize]));
}
