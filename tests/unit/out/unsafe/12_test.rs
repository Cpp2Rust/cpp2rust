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
    let mut v: Vec<Vec<i32>> = Vec::new();
    v.push(
        (0..(10_u64) as usize)
            .map(|_| <i32>::default())
            .collect::<Vec<_>>(),
    );
    return 0;
}
