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
    let mut buf: [libc::c_char; 1] = [(0 as libc::c_char); 1];
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            1_usize
        )) == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_lseek_1() {
    let mut path: *const libc::c_char =
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
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            5_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
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
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_read_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut fd: i32 = libc::fileno(fp);
    let mut buf: [libc::c_char; 16] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            16_usize
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                11_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"hello world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
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
    let mut path: *const libc::c_char =
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
    let mut msg: *const libc::c_char = (c"world".as_ptr().cast_mut()).cast_const();
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (msg as *const libc::c_char as *const ::libc::c_void),
            5_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            8_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (msg as *const libc::c_char as *const ::libc::c_void) as *const u8,
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
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            8_usize
        )) == (0_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_ftruncate_5() {
    let mut path: *const libc::c_char =
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
pub unsafe fn test_open_6() {
    let mut fd: i32 = (unsafe {
        libc::open(
            (c"/dev/null".as_ptr().cast_mut()).cast_const() as *const i8,
            0 as i32,
            (420),
        )
    });
    assert!(((((fd) >= (-1_i32)) as i32) != 0));
    if ((((fd) >= (0)) as i32) != 0) {
        libc::close(fd);
    }
    fd = (unsafe {
        libc::open(
            (c"/dev/null".as_ptr().cast_mut()).cast_const() as *const i8,
            0 as i32,
        )
    });
    assert!(((((fd) >= (-1_i32)) as i32) != 0));
    if ((((fd) >= (0)) as i32) != 0) {
        libc::close(fd);
    }
}
pub unsafe fn test_fcntl_7() {
    assert!(((((unsafe { libc::fcntl(0 as i32, 1 as i32,) }) >= (-1_i32)) as i32) != 0));
    let mut duped: i32 = (unsafe { libc::fcntl(0 as i32, 0 as i32, (100)) });
    assert!(((((duped) >= (-1_i32)) as i32) != 0));
    if ((((duped) >= (0)) as i32) != 0) {
        libc::close(duped);
    }
}
pub unsafe fn test_ioctl_8() {
    let mut arg: i32 = 0;
    assert!(
        ((((unsafe { libc::ioctl(0 as i32, 0_u64 as u64, (&mut arg as *mut i32),) }) >= (-1_i32))
            as i32)
            != 0)
    );
}
pub unsafe fn test_isatty_9() {
    printf(
        (c"%d\n".as_ptr().cast_mut()).cast_const() as *const i8,
        libc::isatty(0),
    );
}
pub unsafe fn test_geteuid_10() {
    printf(
        (c"%u\n".as_ptr().cast_mut()).cast_const() as *const i8,
        libc::geteuid(),
    );
}
pub unsafe fn test_gethostname_11() {
    let mut name: [libc::c_char; 256] = [(0 as libc::c_char); 256];
    assert!(
        ((((libc::gethostname(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>()
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
    (unsafe { test_open_6() });
    (unsafe { test_fcntl_7() });
    (unsafe { test_ioctl_8() });
    (unsafe { test_isatty_9() });
    (unsafe { test_geteuid_10() });
    (unsafe { test_gethostname_11() });
    return 0;
}
