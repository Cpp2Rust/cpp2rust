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
    v.push(1);
    v.push(2);
    v.push(3);
    let mut v_begin: *mut i32 = v.as_mut_ptr();
    let mut v_end: *mut i32 = v.as_mut_ptr().add(v.len());
    let mut it: *mut i32 = {
        let mut it = v_begin;
        while it != v_end && *it != 2 {
            it = it.add(1);
        }
        it
    };
    let mut v_result_true: bool = it != v.as_mut_ptr().add(v.len());
    let mut m: BTreeMap<i32, Box<(i32, f64)>> = BTreeMap::new();
    (*&mut m
        .entry(1.clone())
        .or_insert_with(|| Box::new((1, f64::default())))
        .1) = 1_f64;
    (*&mut m
        .entry(2.clone())
        .or_insert_with(|| Box::new((2, f64::default())))
        .1) = 2_f64;
    (*&mut m
        .entry(3.clone())
        .or_insert_with(|| Box::new((3, f64::default())))
        .1) = 3_f64;
    let mut m_begin: UnsafeMapIterator<i32, f64> =
        UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<(i32, f64)>>);
    let mut m_end: UnsafeMapIterator<i32, f64> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i32, Box<(i32, f64)>>);
    let mut m_result_true: bool = m_begin != m_end;
    assert!(
        ((v_result_true) && (m_result_true))
            && ({
                let mut it = v.as_mut_ptr();
                while it != v.as_mut_ptr() && *it != 2 {
                    it = it.add(1);
                }
                it
            } == v.as_mut_ptr())
    );
    let mut w: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert!(
        {
            let mut it = w.as_mut_ptr().add(2_i64 as usize);
            while it != w.as_mut_ptr().add(w.len()) && *it != 4 {
                it = it.add(1);
            }
            it
        } == w.as_mut_ptr().add(3_i64 as usize)
    );
    assert!(
        {
            let mut it = w.as_mut_ptr().add(2_i64 as usize);
            while it != w.as_mut_ptr().add(w.len()) && *it != 1 {
                it = it.add(1);
            }
            it
        } == w.as_mut_ptr().add(w.len())
    );
    assert!(
        {
            let mut it = w.as_mut_ptr().add(2_i64 as usize);
            while it != w.as_mut_ptr().add(4_i64 as usize) && *it != 5 {
                it = it.add(1);
            }
            it
        } == w.as_mut_ptr().add(4_i64 as usize)
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
