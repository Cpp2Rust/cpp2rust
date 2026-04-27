extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub union node_anon_0 {
    pub bytes: [u8; 1],
    pub aligner: *mut ::libc::c_void,
}
impl Default for node_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct node {
    pub len: u64,
    pub x: node_anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut tail_size: u64 = 32_u64;
    let mut n: *mut node = ((unsafe {
        let ___size: u64 = (::std::mem::size_of::<node>() as u64 as u64).wrapping_add(tail_size);
        malloc_0(___size)
    }) as *mut node);
    (*n).len = tail_size;
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (tail_size)) {
        (*n).x.bytes[(i) as usize] = ((((i) & (255_u64)) as u64) as u8);
        i.postfix_inc();
    }
    let mut i: u64 = 0_u64;
    'loop_: while ((i) < (tail_size)) {
        assert!(
            (((*n).x.bytes[(i) as usize] as i32) == (((((i) & (255_u64)) as u64) as u8) as i32))
        );
        i.postfix_inc();
    }
    let mut p: *mut u8 = (&mut (*n).x.bytes[(10) as usize] as *mut u8);
    assert!((((*p) as i32) == (10)));
    (*p) = 170_u8;
    assert!((((*n).x.bytes[(10) as usize] as i32) == (170)));
    (unsafe {
        let ___ptr: *mut ::libc::c_void = (n as *mut node as *mut ::libc::c_void);
        free_1(___ptr)
    });
    return 0;
}
