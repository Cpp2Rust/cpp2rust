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
    assert!(
        ((((libc::poll(pfd.as_mut_ptr(), (2 as ::libc::nfds_t) as ::libc::nfds_t, 0)) == (1))
            as i32)
            != 0)
    );
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
    return 0;
}
