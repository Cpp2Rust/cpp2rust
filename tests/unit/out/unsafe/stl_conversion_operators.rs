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
    let mut n: u32 = 17_u32;
    let mut d: u32 = n;
    assert!(((d) == (17_u32)));
    let mut name: *const libc::c_char = c"dir/file.txt".as_ptr();
    let mut p: Vec<libc::c_char> = {
        let mut __v = ::std::ffi::CStr::from_ptr(name)
            .to_bytes()
            .iter()
            .map(|&__b| __b as libc::c_char)
            .collect::<Vec<libc::c_char>>();
        __v.push(0);
        __v
    };
    let mut s: Vec<libc::c_char> = p;
    assert!(((s.len() - 1) == (12_usize)));
    assert!(
        s == {
            let s = c"dir/file.txt".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
