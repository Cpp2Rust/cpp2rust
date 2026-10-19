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
    let mut v: Vec<Vec<i32>> = Vec::new();
    {
        let __a0 = 2_usize as usize;
        v.resize_with(__a0, || <Vec<i32>>::default())
    };
    {
        let __a0 = 2_usize as usize;
        v[(0_usize)].resize_with(__a0, || <i32>::default())
    };
    {
        let __a0 = 1_usize as usize;
        v[(1_usize)].resize_with(__a0, || <i32>::default())
    };
    v[(0_usize)][(0_usize)] = 1;
    v[(0_usize)][(1_usize)] = 5;
    v[(1_usize)][(0_usize)] = 6;
    'loop_: for v2 in IterRange::new(v.begin(), v.end()) {
        let v2: *mut Vec<i32> = &mut (*v2.iter_deref());
        'loop_: for i in IterRange::new((*v2).begin(), (*v2).end()) {
            let i: *mut i32 = &mut (*i.iter_deref());
            (*i).prefix_inc();
        }
    }
    'loop_: for v2 in IterRange::new(v.begin(), v.end()) {
        let v2: *mut Vec<i32> = &mut (*v2.iter_deref());
        'loop_: for i in IterRange::new((*v2).begin(), (*v2).end()) {
            let mut i: i32 = (*i.iter_deref());
            (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, ((i) + (3))) });
        }
    }
    'loop_: for v2 in IterRange::new(v.begin(), v.end()) {
        let mut v2: Vec<i32> = (*v2.iter_deref()).clone();
        'loop_: for i in IterRange::new(v2.begin(), v2.end()) {
            let mut i: i32 = (*i.iter_deref());
            (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (i)) });
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
