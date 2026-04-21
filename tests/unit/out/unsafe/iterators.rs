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
    let mut x: Vec<u8> = {
        let s = b"hello\0".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    'loop_: for c in 0..(x.len() - 1) {
        let mut c = x.as_mut_ptr().add(c);
        (*c).prefix_inc();
    }
    'loop_: for c in 0..(x.len() - 1) {
        let mut c = x.as_mut_ptr().add(c);
        printf(b"%c\n\0".as_ptr() as *const i8, ((*c) as i32));
    }
    'loop_: for c in 0..(x.len() - 1) {
        let mut c = x[c].clone();
        printf(b"%c\n\0".as_ptr() as *const i8, (c as i32));
    }
    let mut v: Vec<*mut i32> = Vec::new();
    v.push((Box::leak(Box::new(2)) as *mut i32));
    v.push((Box::leak(Box::new(3)) as *mut i32));
    'loop_: for p in 0..(v.len()) {
        let mut p = v[p].clone();
        printf(b"%d\n\0".as_ptr() as *const i8, (*p));
    }
    return 0;
}
