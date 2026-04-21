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
    v.resize_with(2_u64 as usize, || <Vec<i32>>::default());
    {
        let __a0 = 2_u64 as usize;
        v[(0_u64) as usize].resize_with(__a0, || <i32>::default())
    };
    {
        let __a0 = 1_u64 as usize;
        v[(1_u64) as usize].resize_with(__a0, || <i32>::default())
    };
    v[(0_u64) as usize][(0_u64) as usize] = 1;
    v[(0_u64) as usize][(1_u64) as usize] = 5;
    v[(1_u64) as usize][(0_u64) as usize] = 6;
    'loop_: for v2 in 0..(v.len()) {
        let mut v2 = v.as_mut_ptr().add(v2);
        'loop_: for i in 0..((*v2).len()) {
            let mut i = (*v2).as_mut_ptr().add(i);
            (*i).prefix_inc();
        }
    }
    'loop_: for v2 in 0..(v.len()) {
        let mut v2 = v.as_mut_ptr().add(v2);
        'loop_: for i in 0..((*v2).len()) {
            let mut i = (*v2)[i].clone();
            printf(b"%d\n\0".as_ptr() as *const i8, ((i) + (3)));
        }
    }
    'loop_: for v2 in 0..(v.len()) {
        let mut v2 = v[v2].clone();
        'loop_: for i in 0..(v2.len()) {
            let mut i = v2[i].clone();
            printf(b"%d\n\0".as_ptr() as *const i8, i);
        }
    }
    return 0;
}
