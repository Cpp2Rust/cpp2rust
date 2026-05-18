extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut p: *mut i32) -> *mut i32 {
    return (&mut (*p.offset((5) as isize)) as *mut i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: *mut i32 =
        Box::leak((0..10_u64).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (10_u32)) {
        (*p1.offset((i) as isize)) = (i as i32);
        i.prefix_inc();
    }
    let mut out: i32 = (*(unsafe {
        let _p: *mut i32 = (&mut (*p1.offset((1) as isize)) as *mut i32);
        foo_0(_p)
    })
    .offset((3) as isize));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        p1,
        libcc2rs::malloc_usable_size(p1 as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return out;
}
