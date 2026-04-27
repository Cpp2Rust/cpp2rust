extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Item {
    pub value: i32,
}
impl Item {
    pub unsafe fn foo(&mut self, mut other: *mut Item) {
        (*other).value = 10;
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: *mut Item = Box::leak(
        (0..2_u64)
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    )
    .as_mut_ptr();
    (*arr.offset((0) as isize)).value = 1;
    (*arr.offset((1) as isize)).value = 2;
    (unsafe {
        let _other: *mut Item = (&mut (*arr.offset((1) as isize)) as *mut Item);
        (*arr.offset((0) as isize)).foo(_other)
    });
    let mut result: i32 =
        (((*arr.offset((0) as isize)).value) + ((*arr.offset((1) as isize)).value));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        arr,
        libcc2rs::malloc_usable_size(arr as *mut ::libc::c_void) / ::std::mem::size_of::<Item>(),
    )));
    return result;
}
