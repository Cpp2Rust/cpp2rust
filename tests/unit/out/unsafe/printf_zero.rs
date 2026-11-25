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
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe {
        libc::printf(
            (c"%e|%E\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (0_f64),
            (-0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"%.0e|%#.0E|%02.2e\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (0_f64),
            (-0_f64),
            (0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"%+ 012.2e|%-#012.0E|% 12.1e\n".as_ptr().cast_mut()).cast_const()
                as *const libc::c_char,
            (0_f64),
            (-0_f64),
            (0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"% .1e|% 012.1e\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (0_f64),
            (0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"%g|%G|%#g\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (0_f64),
            (-0_f64),
            (0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"%.0g|%.1g|%#.0g|%#.1G|%#.4g\n".as_ptr().cast_mut()).cast_const()
                as *const libc::c_char,
            (0_f64),
            (-0_f64),
            (0_f64),
            (-0_f64),
            (0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"%+ 08g|% g|% 08G|%-#08.4G|%02.4g\n".as_ptr().cast_mut()).cast_const()
                as *const libc::c_char,
            (0_f64),
            (0_f64),
            (0_f64),
            (-0_f64),
            (0_f64),
        )
    });
    (unsafe {
        libc::printf(
            (c"%f|%F|%e|%g\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (0_f64),
            (-0_f64),
            (1500_f64),
            (0.5_f64),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
