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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    let mut rset: ::libc::fd_set = std::mem::zeroed::<::libc::fd_set>();
    libc::FD_ZERO((&mut rset as *mut ::libc::fd_set));
    libc::FD_SET(fds[(0) as usize], (&mut rset as *mut ::libc::fd_set));
    let mut tv: ::libc::timeval = unsafe { std::mem::zeroed() };
    {
        let byte_0 = ((&mut tv as *mut ::libc::timeval) as *mut ::libc::timeval
            as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<::libc::timeval>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut tv as *mut ::libc::timeval) as *mut ::libc::timeval as *mut ::libc::c_void)
    };
    tv.tv_sec = 0_i64;
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
    return 0;
}
