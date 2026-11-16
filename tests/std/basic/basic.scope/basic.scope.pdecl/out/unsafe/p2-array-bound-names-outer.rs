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
    let length: i32 = 2;
    {
        let mut length: [i32; 2] = [7, 8];
        assert!(
            (((::std::mem::size_of::<[i32; 2]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                == (2_usize))
        );
        assert!(((length[(0) as usize]) == (7)));
        assert!(((length[(1) as usize]) == (8)));
    }
    assert!(((length) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
