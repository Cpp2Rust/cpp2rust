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
    let mut input: [i32; 3] = [1, 2, 3];
    let mut output: [i32; 3] = [0_i32; 3];
    {
        let mut __first = input.as_mut_ptr();
        let mut __out = output.as_mut_ptr();
        while __first != input.as_mut_ptr().offset((3) as isize) {
            *__out.iter_deref() = (*__first.iter_deref()).clone().into();
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
