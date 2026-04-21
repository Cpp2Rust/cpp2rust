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
    let mut arr: Vec<u8> = {
        let s = b"foo\0".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    arr[(1_u64) as usize] = ('b' as u8);
    let mut p: *const u8 = arr.as_ptr().offset((1) as isize);
    assert!((((*p) as i32) == (('b' as u8) as i32)));
    assert!(
        arr == {
            let s = b"fbo\0".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    return 0;
}
