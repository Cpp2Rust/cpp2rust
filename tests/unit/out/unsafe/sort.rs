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
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(1);
    v.push(9);
    v.push(2);
    v.push(8);
    v.push(3);
    v.push(7);
    v.push(4);
    v.push(5);
    v.push(6);
    {
        let len = v.as_mut_ptr().add(v.len()).offset_from(v.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(v.as_mut_ptr(), len).sort()
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ((v.len() as u64).wrapping_sub(1_u64))) {
        assert!(((v[(i as u64) as usize]) < (v[(((i).wrapping_add(1_u32)) as u64) as usize])));
        i.prefix_inc();
    }
    return 0;
}
