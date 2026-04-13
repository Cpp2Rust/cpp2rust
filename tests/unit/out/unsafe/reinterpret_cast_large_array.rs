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
    let N: i32 = 10000;
    let mut arr: *mut u32 =
        Box::leak((0..(N as u64)).map(|_| 0 as u32).collect::<Box<[u32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*arr.offset((i) as isize)) = 0_u32;
        i.postfix_inc();
    }
    (*arr.offset(((N) - (1)) as isize)) = 3148519816_u32;
    let mut words: *mut u16 = (arr as *mut u16);
    assert!((((*words.offset((((N) * (2)) - (1)) as isize)) as i32) == (48042)));
    assert!((((*words.offset((((N) * (2)) - (2)) as isize)) as i32) == (39304)));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        arr,
        libcc2rs::malloc_usable_size(arr as *mut ::libc::c_void) / ::std::mem::size_of::<u32>(),
    )));
    return 0;
}
