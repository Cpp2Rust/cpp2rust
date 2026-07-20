extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_open_read_write_0() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_fd_io_rw.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            (((::libc::O_WRONLY) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fd,
            (c"hello world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            11_usize
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    fd = (unsafe { libc::open(path as *const i8, ::libc::O_RDONLY as i32) });
    assert!(((((fd) >= (0)) as i32) != 0));
    let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
    {
        let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[libc::c_char; 16]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 16]>()
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"hello world".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 16]>()
        )) == (0_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_pipe_1() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (c"ab".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            2_usize
        )) == (2_isize)) as i32)
            != 0)
    );
    let mut buf: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    {
        let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[libc::c_char; 4]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 4]>()
        )) == (2_isize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"ab".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 4]>()
        )) == (0_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_socket_listen_2() {
    let mut s: i32 = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    assert!(((((s) >= (0)) as i32) != 0));
    assert!(((((libc::listen(s, 5)) == (0)) as i32) != 0));
    assert!(((((libc::close(s)) == (0)) as i32) != 0));
}
pub unsafe fn test_lseek_3() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_fd_io_lseek.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            (((::libc::O_RDWR) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fd,
            (c"hello world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            11_usize
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(((((libc::lseek(fd, 0_i64, 2)) == (11_i64)) as i32) != 0));
    assert!(((((libc::lseek(fd, 6_i64, 0)) == (6_i64)) as i32) != 0));
    let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
    {
        let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[libc::c_char; 16]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 16]>()
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"world".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_ftruncate_4() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_fd_io_trunc.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            (((::libc::O_RDWR) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fd,
            (c"hello world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            11_usize
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(((((libc::ftruncate(fd, 5_i64)) == (0)) as i32) != 0));
    assert!(((((libc::lseek(fd, 0_i64, 2)) == (5_i64)) as i32) != 0));
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_fstat_5() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_fd_io_fstat.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            (((::libc::O_WRONLY) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fd,
            (c"hello".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            5_usize
        )) == (5_isize)) as i32)
            != 0)
    );
    let mut st: ::libc::stat = unsafe { std::mem::zeroed() };
    assert!(((((libc::fstat(fd, (&mut st as *mut ::libc::stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (5_i64)) as i32) != 0));
    assert!((((((st.st_mode) & (61440_u32)) == (32768_u32)) as i32) != 0));
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_isatty_6() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_fd_io_tty.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            ((::libc::O_RDONLY) | (::libc::O_CREAT)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    assert!(((((libc::isatty(fd)) == (0)) as i32) != 0));
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_tcgetattr_7() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_fd_io_termios.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            ((::libc::O_RDONLY) | (::libc::O_CREAT)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    let mut tio: ::libc::termios = unsafe { std::mem::zeroed() };
    assert!(
        ((((libc::tcgetattr(fd, (&mut tio as *mut ::libc::termios))) == (-1_i32)) as i32) != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_select_8() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    let mut rset: ::libc::fd_set = std::mem::zeroed::<::libc::fd_set>();
    libc::FD_ZERO((&mut rset as *mut ::libc::fd_set));
    libc::FD_SET(fds[(0) as usize], (&mut rset as *mut ::libc::fd_set));
    let mut tv: ::libc::timeval = unsafe { std::mem::zeroed() };
    tv.tv_sec = 0_i64;
    tv.tv_usec = 0_i64;
    assert!(
        ((((libc::select(
            ((fds[(0) as usize]) + (1)),
            (&mut rset as *mut ::libc::fd_set),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut tv as *mut ::libc::timeval)
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((!(libc::FD_ISSET(
            fds[(0) as usize],
            (&mut rset as *mut ::libc::fd_set).cast_const()
        ) as i32
            != 0) as i32)
            != 0)
    );
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (c"x".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            1_usize
        )) == (1_isize)) as i32)
            != 0)
    );
    libc::FD_ZERO((&mut rset as *mut ::libc::fd_set));
    libc::FD_SET(fds[(0) as usize], (&mut rset as *mut ::libc::fd_set));
    tv.tv_sec = 1_i64;
    tv.tv_usec = 0_i64;
    assert!(
        ((((libc::select(
            ((fds[(0) as usize]) + (1)),
            (&mut rset as *mut ::libc::fd_set),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut tv as *mut ::libc::timeval)
        )) == (1)) as i32)
            != 0)
    );
    assert!(
        (libc::FD_ISSET(
            fds[(0) as usize],
            (&mut rset as *mut ::libc::fd_set).cast_const()
        ) as i32
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_poll_9() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (c"x".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            1_usize
        )) == (1_isize)) as i32)
            != 0)
    );
    let mut pfd: [::libc::pollfd; 2] = [unsafe { std::mem::zeroed() }; 2];
    pfd[(0) as usize].fd = fds[(0) as usize];
    pfd[(0) as usize].events = 1_i16;
    pfd[(0) as usize].revents = 0_i16;
    pfd[(1) as usize].fd = -1_i32;
    pfd[(1) as usize].events = 1_i16;
    pfd[(1) as usize].revents = 42_i16;
    assert!(((((libc::poll(pfd.as_mut_ptr(), 2_u64 as ::libc::nfds_t, 0)) == (1)) as i32) != 0));
    assert!((((((pfd[(0) as usize].revents as i32) & (1)) != (0)) as i32) != 0));
    assert!(((((pfd[(1) as usize].revents as i32) == (0)) as i32) != 0));
    let mut ch: libc::c_char = (0 as libc::c_char);
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            ((&mut ch as *mut libc::c_char) as *mut libc::c_char as *mut ::libc::c_void),
            1_usize
        )) == (1_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_open_read_write_0() });
    (unsafe { test_pipe_1() });
    (unsafe { test_socket_listen_2() });
    (unsafe { test_lseek_3() });
    (unsafe { test_ftruncate_4() });
    (unsafe { test_fstat_5() });
    (unsafe { test_isatty_6() });
    (unsafe { test_tcgetattr_7() });
    (unsafe { test_select_8() });
    (unsafe { test_poll_9() });
    return 0;
}
