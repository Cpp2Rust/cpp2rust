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
    let mut x: Vec<libc::c_char> = {
        let s = c"hello".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    'loop_: for c in UnsafeIterRange::new(x.as_mut_ptr(), x.as_mut_ptr().add(x.len() - 1)) {
        let c: *mut libc::c_char = &mut (*c.iter_deref());
        (*c).prefix_inc();
    }
    'loop_: for c in UnsafeIterRange::new(x.as_mut_ptr(), x.as_mut_ptr().add(x.len() - 1)) {
        let c: *mut libc::c_char = &mut (*c.iter_deref());
        (unsafe { libc::printf(c"%c\n".as_ptr() as *const libc::c_char, ((*c) as i32)) });
    }
    'loop_: for c in UnsafeIterRange::new(x.as_mut_ptr(), x.as_mut_ptr().add(x.len() - 1)) {
        let mut c: libc::c_char = (*c.iter_deref());
        (unsafe { libc::printf(c"%c\n".as_ptr() as *const libc::c_char, (c as i32)) });
    }
    let mut v: Vec<*mut i32> = Vec::new();
    {
        let __a1 = (Box::leak(Box::new(2)) as *mut i32);
        v.push(__a1)
    };
    {
        let __a1 = (Box::leak(Box::new(3)) as *mut i32);
        v.push(__a1)
    };
    'loop_: for p in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
    ) {
        let mut p: *mut i32 = (*p.iter_deref());
        (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (*p)) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
