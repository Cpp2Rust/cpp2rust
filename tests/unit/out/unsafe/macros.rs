extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn base_name_0(mut path: *const libc::c_char) -> *const libc::c_char {
    let mut slash: *const libc::c_char =
        (libc::strrchr(path, (('/' as libc::c_char) as i32)) as *const libc::c_char);
    return if !(slash).is_null() {
        slash.offset((1) as isize)
    } else {
        path
    };
}
pub unsafe fn log_1(mut file: *const libc::c_char, mut line: i32, mut func: *const libc::c_char) {
    printf(c"%s %d %s\n".as_ptr() as *const i8, file, line, func);
}
pub unsafe fn line_2() -> i32 {
    return (line!() as u32 as i32);
}
pub unsafe fn function_3() -> *const libc::c_char {
    return c"function".as_ptr();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    printf(
        c"%s %d %s\n".as_ptr() as *const i8,
        c"macros.cpp".as_ptr(),
        19,
        c"main".as_ptr(),
    );
    (unsafe { log_1(c"macros.cpp".as_ptr(), 20, c"main".as_ptr()) });
    assert!(((line!() as u32) > (0_u32)));
    assert!(
        (((*(unsafe { base_name_0(concat!(file!(), "\0").as_ptr() as *const libc::c_char,) })
            .offset((0) as isize)) as i32)
            != (('\0' as libc::c_char) as i32))
    );
    assert!(((unsafe { line_2() }) > (0)));
    printf(c"%s\n".as_ptr() as *const i8, (unsafe { function_3() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
