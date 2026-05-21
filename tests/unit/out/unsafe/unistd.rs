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
    let mut buf: [u8; 1] = [0_u8; 1];
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            1_u64 as usize
        ) as i64)
            == (-1_i32 as i64)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_lseek_1() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_lseek_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(
        path as *const i8,
        (b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs(
        (b"hello world\0".as_ptr().cast_mut()).cast_const() as *const i8,
        fp,
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(
        path as *const i8,
        (b"rb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut fd: i32 = libc::fileno(fp);
    assert!(((((libc::lseek(fd, 0_i64, 2)) == (11_i64)) as i32) != 0));
    assert!(((((libc::lseek(fd, 6_i64, 0)) == (6_i64)) as i32) != 0));
    let mut buf: [u8; 8] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            5_u64 as usize
        ) as i64)
            == (5_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (b"world\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
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
    libc::unlink(path as *const i8);
}
pub unsafe fn test_read_2() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_read_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(
        path as *const i8,
        (b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs(
        (b"hello world\0".as_ptr().cast_mut()).cast_const() as *const i8,
        fp,
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(
        path as *const i8,
        (b"rb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut fd: i32 = libc::fileno(fp);
    let mut buf: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            16_u64 as usize
        ) as i64)
            == (11_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                11_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (b"hello world\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void)
                    as *const u8,
                11_u64 as usize,
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
    libc::unlink(path as *const i8);
}
pub unsafe fn test_unlink_3() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_unlink_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(
        path as *const i8,
        (b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path as *const i8)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path as *const i8)) == (-1_i32)) as i32) != 0));
}
pub unsafe fn test_pipe_4() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    let mut msg: *const u8 = (b"world\0".as_ptr().cast_mut()).cast_const();
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (msg as *const u8 as *const ::libc::c_void),
            5_u64 as usize
        ) as i64)
            == (5_i64)) as i32)
            != 0)
    );
    let mut buf: [u8; 8] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            8_u64 as usize
        ) as i64)
            == (5_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (msg as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
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
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            8_u64 as usize
        ) as i64)
            == (0_i64)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_ftruncate_5() {
    let mut path: *const u8 =
        (b"/tmp/cpp2rust_ftruncate_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(
        path as *const i8,
        (b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs(
        (b"hello world\0".as_ptr().cast_mut()).cast_const() as *const i8,
        fp,
    );
    libc::fflush(fp);
    let mut fd: i32 = libc::fileno(fp);
    assert!(((((libc::ftruncate(fd, 5_i64)) == (0)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(
        path as *const i8,
        (b"rb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    fd = (libc::fileno(fp)).clone();
    assert!(((((libc::lseek(fd, 0_i64, 2)) == (5_i64)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path as *const i8);
}
pub unsafe fn test_isatty_6() {
    printf(
        (b"%d\n\0".as_ptr().cast_mut()).cast_const() as *const i8,
        libc::isatty(0),
    );
}
pub unsafe fn test_geteuid_7() {
    printf(
        (b"%u\n\0".as_ptr().cast_mut()).cast_const() as *const i8,
        libc::geteuid(),
    );
}
pub unsafe fn test_gethostname_8() {
    let mut name: [u8; 256] = [0_u8; 256];
    assert!(
        ((((libc::gethostname(
            name.as_mut_ptr() as *mut i8,
            ::std::mem::size_of::<[u8; 256]>() as u64 as usize
        )) == (0)) as i32)
            != 0)
    );
    printf(
        (b"%s\n\0".as_ptr().cast_mut()).cast_const() as *const i8,
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
