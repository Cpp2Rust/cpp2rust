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
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
    ) {
        let mut e1: i32 = (*__begin1.iter_deref());
        'loop_: for __begin2 in UnsafeIterRange::new(
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
        ) {
            let mut e2: i32 = (*__begin2.iter_deref());
            square += ((e1) * (e2));
        }
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
    ) {
        let e1: *mut i32 = &mut (*__begin1.iter_deref());
        'loop_: for __begin2 in UnsafeIterRange::new(
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
        ) {
            let e2: *const i32 = &(*__begin2.iter_deref());
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
    ) {
        let e1: *const i32 = &(*__begin1.iter_deref());
        'loop_: for __begin2 in UnsafeIterRange::new(
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
        ) {
            let e2: *mut i32 = &mut (*__begin2.iter_deref());
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
    ) {
        let e1: *mut i32 = &mut (*__begin1.iter_deref());
        'loop_: for __begin2 in UnsafeIterRange::new(
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()),
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len())),
        ) {
            let e2: *mut i32 = &mut (*__begin2.iter_deref());
            square += ((*e1) * (*e2));
        }
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
    'loop_: for __begin1 in UnsafeIterRange::new(
        UnsafeRandomAccessIterator::Vec(m.as_mut_ptr()),
        UnsafeRandomAccessIterator::Vec(m.as_mut_ptr().add(m.len())),
    ) {
        let row: *mut Vec<i32> = &mut (*__begin1.iter_deref());
        'loop_: for __begin2 in UnsafeIterRange::new(
            UnsafeRandomAccessIterator::Vec((*row).as_mut_ptr()),
            UnsafeRandomAccessIterator::Vec((*row).as_mut_ptr().add((*row).len())),
        ) {
            let col: *mut i32 = &mut (*__begin2.iter_deref());
            square += (*col);
        }
    }
    assert!(((square) == (144)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
