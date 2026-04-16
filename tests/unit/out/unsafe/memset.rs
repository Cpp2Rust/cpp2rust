extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: i32 = 3;
    let mut arr: *mut i32 =
        Box::leak((0..(N as u64)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    {
        let byte_0 = (arr as *mut i32 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..(::std::mem::size_of::<i32>() as u64 as u64).wrapping_mul((N as u64)) {
            *byte_0.offset(offset as isize) = 1 as u8;
        }
        (arr as *mut i32 as *mut ::libc::c_void)
    };
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        sum += (*arr.offset((i) as isize));
        i.prefix_inc();
    }

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        arr,
        libcc2rs::malloc_usable_size(arr as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return sum;
}
