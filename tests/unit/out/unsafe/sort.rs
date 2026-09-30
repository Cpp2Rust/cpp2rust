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
    let mut v: Vec<i32> = Vec::new();
    {
        let __a1 = 10;
        v.push(__a1)
    };
    {
        let __a1 = 1;
        v.push(__a1)
    };
    {
        let __a1 = 9;
        v.push(__a1)
    };
    {
        let __a1 = 2;
        v.push(__a1)
    };
    {
        let __a1 = 8;
        v.push(__a1)
    };
    {
        let __a1 = 3;
        v.push(__a1)
    };
    {
        let __a1 = 7;
        v.push(__a1)
    };
    {
        let __a1 = 4;
        v.push(__a1)
    };
    {
        let __a1 = 5;
        v.push(__a1)
    };
    {
        let __a1 = 6;
        v.push(__a1)
    };
    {
        let len = v.as_mut_ptr().add(v.len()).offset_from(v.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(v.as_mut_ptr(), len).sort()
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((v.len()).wrapping_sub(1_usize))) {
        assert!(((v[(i as usize)]) < (v[(((i).wrapping_add(1_u32)) as usize)])));
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
