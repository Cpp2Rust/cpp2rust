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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fd: i32 = 0;
    'loop_: while ((((fd) < (3)) as i32) != 0) {
        assert!(((((libc::close(fd)) == (0)) as i32) != 0));
        let mut reopened: i32 = (unsafe {
            libc::open(
                (c"/dev/zero".as_ptr().cast_mut()).cast_const() as *const i8,
                ::libc::O_RDONLY as i32,
            )
        });
        assert!(((((reopened) == (fd)) as i32) != 0));
        let mut buf: [u8; 3] = [1_u8, 127_u8, 255_u8];
        assert!(
            ((((libc::read(
                reopened,
                (buf.as_mut_ptr() as *mut ::libc::c_void),
                ::std::mem::size_of::<[u8; 3]>()
            ) as usize)
                == (::std::mem::size_of::<[u8; 3]>())) as i32)
                != 0)
        );
        assert!(
            ((((((((((buf[(0) as usize] as i32) == (0)) as i32) != 0)
                && ((((buf[(1) as usize] as i32) == (0)) as i32) != 0)) as i32)
                != 0)
                && ((((buf[(2) as usize] as i32) == (0)) as i32) != 0)) as i32)
                != 0)
        );
        fd.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
