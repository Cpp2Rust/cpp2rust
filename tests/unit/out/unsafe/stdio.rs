extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_fputc_0() {
    libc::fputc(('H' as i32), libcc2rs::stdout_unsafe());
    libc::fputc(('i' as i32), libcc2rs::stdout_unsafe());
    libc::fputc(('\n' as i32), libcc2rs::stdout_unsafe());
}
pub unsafe fn test_fputs_1() {
    libc::fputs(
        (c"hello".as_ptr().cast_mut()).cast_const(),
        libcc2rs::stdout_unsafe(),
    );
    libc::fputc(('\n' as i32), libcc2rs::stdout_unsafe());
    let mut s: *const libc::c_char = (c"from variable".as_ptr().cast_mut()).cast_const();
    libc::fputs(s, libcc2rs::stdout_unsafe());
    libc::fputc(('\n' as i32), libcc2rs::stdout_unsafe());
    let mut buf: [libc::c_char; 4] = [
        (('b' as i32) as libc::c_char),
        (('u' as i32) as libc::c_char),
        (('f' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    libc::fputs((buf.as_mut_ptr()).cast_const(), libcc2rs::stdout_unsafe());
    libc::fputc(('\n' as i32), libcc2rs::stdout_unsafe());
}
pub unsafe fn test_puts_2() {
    libc::puts((c"puts hello".as_ptr().cast_mut()).cast_const());
    let mut s: *const libc::c_char = (c"puts variable".as_ptr().cast_mut()).cast_const();
    libc::puts(s);
}
pub unsafe fn test_fileno_3() {
    assert!(((((libc::fileno(libcc2rs::stdin_unsafe())) == (0)) as i32) != 0));
    assert!(((((libc::fileno(libcc2rs::stdout_unsafe())) == (1)) as i32) != 0));
    assert!(((((libc::fileno(libcc2rs::stderr_unsafe())) == (2)) as i32) != 0));
    let mut file: *const libc::c_char =
        (c"cpp2rust_fileno_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(file, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fileno(fp)) > (2)) as i32) != 0));
    libc::fclose(fp);
    assert!(((((libc::unlink(file)) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_fputc_0() });
    (unsafe { test_fputs_1() });
    (unsafe { test_puts_2() });
    (unsafe { test_fileno_3() });
    return 0;
}
