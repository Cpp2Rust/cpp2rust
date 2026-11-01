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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: i32 = 3;
    let mut arr: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    match ({ (arr as *mut ::libc::c_void) },) {
        (mut __tmp_0,) => {
            let byte_0 = __tmp_0 as *mut u8;
            for offset in 0..(::std::mem::size_of::<i32>() as usize).wrapping_mul((N as usize)) {
                *byte_0.offset(offset as isize) = 1 as u8;
            }
            __tmp_0
        }
    };
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        sum += (*arr.offset((i) as isize));
        i.prefix_inc();
    }
    {
        let __p = arr;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
    assert!(((sum) == (50529027)));
    let mut bytes: [u8; 4] = [0_u8; 4];
    let mut fill: u8 = 7_u8;
    let mut n: i32 = 2;
    match (
        { ((&mut bytes[(0) as usize] as *mut u8) as *mut ::libc::c_void) },
        { ((fill as i32) + (1)) },
    ) {
        (mut __tmp_1, mut __tmp_2) => {
            let byte_0 = __tmp_1 as *mut u8;
            for offset in 0..(n as usize) {
                *byte_0.offset(offset as isize) = __tmp_2 as u8;
            }
            __tmp_1
        }
    };
    match (
        { ((&mut bytes[(n) as usize] as *mut u8) as *mut ::libc::c_void) },
        { (fill as i32) },
    ) {
        (mut __tmp_3, mut __tmp_4) => {
            let byte_0 = __tmp_3 as *mut u8;
            for offset in 0..(((4) - (n)) as usize) {
                *byte_0.offset(offset as isize) = __tmp_4 as u8;
            }
            __tmp_3
        }
    };
    assert!(
        ((((bytes[(0) as usize] as i32) == (8)) && ((bytes[(1) as usize] as i32) == (8)))
            && ((bytes[(2) as usize] as i32) == (7)))
            && ((bytes[(3) as usize] as i32) == (7))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
