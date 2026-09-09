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
    let mut __tmp_0: i32 = 5;
    let mut r: *const i32 = <*const i32>::default();
    goto_block!({
        '__entry: {
            r = &mut __tmp_0;
            goto!('body);
        }
        'body: {
            assert!(((*r) == (5)));
            return 0;
        }
    });
    panic!("ub: non-void function does not return a value")
}
