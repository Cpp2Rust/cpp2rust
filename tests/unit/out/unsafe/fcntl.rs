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
    let mut flags: i32 = (unsafe { libc::fcntl(fds[(0) as usize] as i32, 3 as i32, (0)) });
    assert!(((((flags) >= (0)) as i32) != 0));
    assert!((((((flags) & (::libc::O_NONBLOCK)) == (0)) as i32) != 0));
    assert!(
        ((((unsafe {
            libc::fcntl(
                fds[(0) as usize] as i32,
                4 as i32,
                ((flags) | (::libc::O_NONBLOCK)),
            )
        }) == (0)) as i32)
            != 0)
    );
    flags = (unsafe { libc::fcntl(fds[(0) as usize] as i32, 3 as i32, (0)) });
    assert!((((((flags) & (::libc::O_NONBLOCK)) != (0)) as i32) != 0));
    let mut b: libc::c_char = (0 as libc::c_char);
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            ((&mut b as *mut libc::c_char) as *mut libc::c_char as *mut ::libc::c_void),
            1_usize
        )) == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(
        (((((unsafe { libc::fcntl(fds[(0) as usize] as i32, 1 as i32, (0),) }) & (1)) == (0))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe { libc::fcntl(fds[(0) as usize] as i32, 2 as i32, (1),) }) == (0)) as i32) != 0)
    );
    assert!(
        (((((unsafe { libc::fcntl(fds[(0) as usize] as i32, 1 as i32, (0),) }) & (1)) != (0))
            as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
    return 0;
}
