extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone)]
pub struct Pointers {
    pub x1: *mut i32,
    pub x2: *const i32,
    pub x3: [*mut i32; 5],
    pub x4: [*const i32; 10],
    pub x5: i32,
}
impl Default for Pointers {
    fn default() -> Self {
        Pointers {
            x1: Default::default(),
            x2: Default::default(),
            x3: [Default::default(); 5],
            x4: [Default::default(); 10],
            x5: 0 as i32,
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut default_pointers: *mut Pointers = Box::leak(
        (0..10_u64)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    )
    .as_mut_ptr();

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        default_pointers,
        libcc2rs::malloc_usable_size(default_pointers as *mut ::libc::c_void)
            / ::std::mem::size_of::<Pointers>(),
    )));
    return 0;
}
