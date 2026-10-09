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
    let mut c: libc::c_char = (('a' as i32) as libc::c_char);
    let mut n: i32 = 3;
    printf(
        (c"%c\n".as_ptr().cast_mut()).cast_const() as *const i8,
        (c as i32),
    );
    printf(
        (c"%d %c\n".as_ptr().cast_mut()).cast_const() as *const i8,
        n,
        (c as i32),
    );
    printf(
        (c"100%% %c\n".as_ptr().cast_mut()).cast_const() as *const i8,
        (c as i32),
    );
    printf(
        (c"%c%c%d\n".as_ptr().cast_mut()).cast_const() as *const i8,
        (c as i32),
        ((c as i32) + (1)),
        n,
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
