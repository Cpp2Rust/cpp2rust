extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_close_0() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    let mut buf: [::core::ffi::c_char; 1] = [(0 as ::core::ffi::c_char); 1];
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut ::core::ffi::c_char as *mut ::libc::c_void),
            1_usize
        )) == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_lseek_1() {
    let mut path: *const ::core::ffi::c_char =
        (c"/tmp/cpp2rust_lseek_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut fd: i32 = libc::fileno(fp);
    assert!(((((libc::lseek(fd, 0_i64, 2)) == (11_i64)) as i32) != 0));
    assert!(((((libc::lseek(fd, 6_i64, 0)) == (6_i64)) as i32) != 0));
    let mut buf: [::core::ffi::c_char; 8] = [
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
    ];
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut ::core::ffi::c_char as *mut ::libc::c_void),
            5_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const ::core::ffi::c_char as *const ::libc::c_void)
                    as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"world".as_ptr().cast_mut() as *const ::core::ffi::c_char
                    as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_read_2() {
    let mut path: *const ::core::ffi::c_char =
        (c"/tmp/cpp2rust_read_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut fd: i32 = libc::fileno(fp);
    let mut buf: [::core::ffi::c_char; 16] = [
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
    ];
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut ::core::ffi::c_char as *mut ::libc::c_void),
            16_usize
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const ::core::ffi::c_char as *const ::libc::c_void)
                    as *const u8,
                11_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"hello world".as_ptr().cast_mut() as *const ::core::ffi::c_char
                    as *const ::libc::c_void) as *const u8,
                11_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_unlink_3() {
    let mut path: *const ::core::ffi::c_char =
        (c"/tmp/cpp2rust_unlink_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (-1_i32)) as i32) != 0));
}
pub unsafe fn test_pipe_4() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    let mut msg: *const ::core::ffi::c_char = (c"world".as_ptr().cast_mut()).cast_const();
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (msg as *const ::core::ffi::c_char as *const ::libc::c_void),
            5_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    let mut buf: [::core::ffi::c_char; 8] = [
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
        (0 as ::core::ffi::c_char),
    ];
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut ::core::ffi::c_char as *mut ::libc::c_void),
            8_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const ::core::ffi::c_char as *const ::libc::c_void)
                    as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (msg as *const ::core::ffi::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut ::core::ffi::c_char as *mut ::libc::c_void),
            8_usize
        )) == (0_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_ftruncate_5() {
    let mut path: *const ::core::ffi::c_char =
        (c"/tmp/cpp2rust_ftruncate_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    libc::fflush(fp);
    let mut fd: i32 = libc::fileno(fp);
    assert!(((((libc::ftruncate(fd, 5_i64)) == (0)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    fd = (libc::fileno(fp)).clone();
    assert!(((((libc::lseek(fd, 0_i64, 2)) == (5_i64)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_isatty_6() {
    printf(
        (c"%d\n".as_ptr().cast_mut()).cast_const() as *const i8,
        libc::isatty(0),
    );
}
pub unsafe fn test_geteuid_7() {
    printf(
        (c"%u\n".as_ptr().cast_mut()).cast_const() as *const i8,
        libc::geteuid(),
    );
}
pub unsafe fn test_gethostname_8() {
    let mut name: [::core::ffi::c_char; 256] = [(0 as ::core::ffi::c_char); 256];
    assert!(
        ((((libc::gethostname(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[::core::ffi::c_char; 256]>()
        )) == (0)) as i32)
            != 0)
    );
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        name.as_mut_ptr(),
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_close_0() });
    (unsafe { test_lseek_1() });
    (unsafe { test_read_2() });
    (unsafe { test_unlink_3() });
    (unsafe { test_pipe_4() });
    (unsafe { test_ftruncate_5() });
    (unsafe { test_isatty_6() });
    (unsafe { test_geteuid_7() });
    (unsafe { test_gethostname_8() });
    return 0;
}
