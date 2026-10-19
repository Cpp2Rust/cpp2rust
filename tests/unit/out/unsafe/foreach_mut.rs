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
    let mut v1: Vec<i32> = Vec::new();
    {
        let __a1 = 1;
        v1.push(__a1)
    };
    {
        let __a1 = 2;
        v1.push(__a1)
    };
    {
        let __a1 = 3;
        v1.push(__a1)
    };
    let mut sum: i32 = 0;
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len())),
    ) {
        let mut x: i32 = (*__begin1.iter_deref());
        sum += x.prefix_inc();
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len())),
    ) {
        let x: i32 = (*__begin1.iter_deref());
        sum += x;
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len())),
    ) {
        let x: *mut i32 = &mut (*__begin1.iter_deref());
        (*x) += 10;
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len())),
    ) {
        let x: *const i32 = &(*__begin1.iter_deref());
        sum += (*x);
    }
    let mut v2: Vec<*mut i32> = Vec::new();
    {
        let __a1 = (&mut v1[(0_usize)] as *mut i32);
        v2.push(__a1)
    };
    {
        let __a1 = (&mut v1[(1_usize)] as *mut i32);
        v2.push(__a1)
    };
    {
        let __a1 = (&mut v1[(2_usize)] as *mut i32);
        v2.push(__a1)
    };
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len())),
    ) {
        let mut p: *mut i32 = (*__begin1.iter_deref());
        (*p) += 5;
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len())),
    ) {
        let p: *mut i32 = (*__begin1.iter_deref());
        sum += (*p);
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len())),
    ) {
        let mut p: *mut i32 = (*__begin1.iter_deref());
        (*p) += 5;
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len())),
    ) {
        let mut p: *const i32 = (*__begin1.iter_deref()).cast_const();
        sum += (*p);
    }
    assert!(((sum) == (168)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
