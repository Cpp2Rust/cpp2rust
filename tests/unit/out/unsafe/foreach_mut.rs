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
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len()));
    'loop_: while __begin1 != __end1 {
        let mut x: i32 = (*__begin1.iter_deref());
        sum += x.prefix_inc();
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len()));
    'loop_: while __begin1 != __end1 {
        let x: i32 = (*__begin1.iter_deref());
        sum += x;
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len()));
    'loop_: while __begin1 != __end1 {
        let x: *mut i32 = &mut (*__begin1.iter_deref());
        (*x) += 10;
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v1.as_mut_ptr().add(v1.len()));
    'loop_: while __begin1 != __end1 {
        let x: *const i32 = &(*__begin1.iter_deref());
        sum += (*x);
        __begin1.iter_inc();
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
    let mut __begin1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len()));
    'loop_: while __begin1 != __end1 {
        let mut p: *mut i32 = (*__begin1.iter_deref());
        (*p) += 5;
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len()));
    'loop_: while __begin1 != __end1 {
        let p: *mut i32 = (*__begin1.iter_deref());
        sum += (*p);
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len()));
    'loop_: while __begin1 != __end1 {
        let mut p: *mut i32 = (*__begin1.iter_deref());
        (*p) += 5;
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<*mut i32> =
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(v2.len()));
    'loop_: while __begin1 != __end1 {
        let mut p: *const i32 = (*__begin1.iter_deref()).cast_const();
        sum += (*p);
        __begin1.iter_inc();
    }
    assert!(((sum) == (168)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
