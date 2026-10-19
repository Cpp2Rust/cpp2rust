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
        let __a1 = 1;
        v.push(__a1)
    };
    {
        let __a1 = 2;
        v.push(__a1)
    };
    {
        let __a1 = 3;
        v.push(__a1)
    };
    let mut square: i32 = 0;
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
    'loop_: while __begin1 != __end1 {
        let mut e1: i32 = (*__begin1.iter_deref());
        let mut __begin2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
        let mut __end2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
        'loop_: while __begin2 != __end2 {
            let mut e2: i32 = (*__begin2.iter_deref());
            square += ((e1) * (e2));
            __begin2.iter_inc();
        }
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
    'loop_: while __begin1 != __end1 {
        let e1: *mut i32 = &mut (*__begin1.iter_deref());
        let mut __begin2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
        let mut __end2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
        'loop_: while __begin2 != __end2 {
            let e2: *const i32 = &(*__begin2.iter_deref());
            square += ((*e1) * (*e2));
            __begin2.iter_inc();
        }
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
    'loop_: while __begin1 != __end1 {
        let e1: *const i32 = &(*__begin1.iter_deref());
        let mut __begin2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
        let mut __end2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
        'loop_: while __begin2 != __end2 {
            let e2: *mut i32 = &mut (*__begin2.iter_deref());
            square += ((*e1) * (*e2));
            __begin2.iter_inc();
        }
        __begin1.iter_inc();
    }
    let mut __begin1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
    'loop_: while __begin1 != __end1 {
        let e1: *mut i32 = &mut (*__begin1.iter_deref());
        let mut __begin2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
        let mut __end2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
        'loop_: while __begin2 != __end2 {
            let e2: *mut i32 = &mut (*__begin2.iter_deref());
            square += ((*e1) * (*e2));
            __begin2.iter_inc();
        }
        __begin1.iter_inc();
    }
    let mut m: Vec<Vec<i32>> = Vec::new();
    let mut v1: Vec<i32> = Vec::new();
    {
        let __a1 = std::mem::take(&mut v1);
        m.push(__a1)
    };
    let mut v2: Vec<i32> = Vec::new();
    {
        let __a1 = std::mem::take(&mut v2);
        m.push(__a1)
    };
    let mut v3: Vec<i32> = Vec::new();
    {
        let __a1 = std::mem::take(&mut v3);
        m.push(__a1)
    };
    let mut __begin1: UnsafeRandomAccessIterator<Vec<i32>> =
        UnsafeRandomAccessIterator::Vec(m.as_mut_ptr());
    let mut __end1: UnsafeRandomAccessIterator<Vec<i32>> =
        UnsafeRandomAccessIterator::Vec(m.as_mut_ptr().add(m.len()));
    'loop_: while __begin1 != __end1 {
        let row: *mut Vec<i32> = &mut (*__begin1.iter_deref());
        let mut __begin2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec((*row).as_mut_ptr());
        let mut __end2: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec((*row).as_mut_ptr().add((*row).len()));
        'loop_: while __begin2 != __end2 {
            let col: *mut i32 = &mut (*__begin2.iter_deref());
            square += (*col);
            __begin2.iter_inc();
        }
        __begin1.iter_inc();
    }
    assert!(((square) == (144)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
