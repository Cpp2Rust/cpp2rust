extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut ptr: *mut ::core::ffi::c_char) -> u32 {
    let mut count: u32 = 0_u32;
    'loop_: while (((*ptr.postfix_inc()) as i32) != (('\0' as ::core::ffi::c_char) as i32)) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut string: [::core::ffi::c_char; 6] = [
        ('h' as ::core::ffi::c_char),
        ('e' as ::core::ffi::c_char),
        ('l' as ::core::ffi::c_char),
        ('l' as ::core::ffi::c_char),
        ('o' as ::core::ffi::c_char),
        ('\0' as ::core::ffi::c_char),
    ];
    return ((unsafe {
        let _ptr: *mut ::core::ffi::c_char =
            (&mut string[(0) as usize] as *mut ::core::ffi::c_char);
        strlen_0(_ptr)
    }) as i32);
}
