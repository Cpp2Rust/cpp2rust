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
pub union anon_0 {
    pub bytes: [u8; 1],
    pub aligner: *mut ::libc::c_void,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct node {
    pub len: usize,
    pub pos: usize,
    pub x: anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut tail_size: usize = 32_usize;
    let mut n: *mut node = (libcc2rs::malloc_unsafe(
        (((::std::mem::size_of::<node>() as u64).wrapping_add(tail_size as u64)) as usize),
    ) as *mut node);
    (*n).len = tail_size;
    let mut i: usize = 0_usize;
    'loop_: while ((((i) < (tail_size)) as i32) != 0) {
        (*(*n).x.bytes.as_mut_ptr().add((i) as usize)) = (((i) & (255_usize)) as u8);
        i.postfix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while ((((i) < (tail_size)) as i32) != 0) {
        assert!(
            (((((*(*n).x.bytes.as_mut_ptr().add((i) as usize)) as i32)
                == ((((i) & (255_usize)) as u8) as i32)) as i32)
                != 0)
        );
        i.postfix_inc();
    }
    let mut p: *mut u8 = ((*n).x.bytes.as_mut_ptr().add((10) as usize));
    assert!((((((*p) as i32) == (10)) as i32) != 0));
    (*p) = 170_u8;
    assert!((((((*(*n).x.bytes.as_mut_ptr().add((10) as usize)) as i32) == (170)) as i32) != 0));
    (*n).pos = 20_usize;
    let mut q: *mut u8 = ((*n).x.bytes.as_mut_ptr().add(((*n).pos) as usize));
    assert!((((((*q) as i32) == (20)) as i32) != 0));
    (*q) = 187_u8;
    assert!((((((*q) as i32) == (187)) as i32) != 0));
    libcc2rs::free_unsafe((n as *mut node as *mut ::libc::c_void));
    return 0;
}
