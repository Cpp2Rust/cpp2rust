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
    let mut m: BTreeMap<i32, Box<f64>> = BTreeMap::new();
    (*m.entry(1).or_default().as_mut()) = 1_f64;
    (*m.entry(2).or_default().as_mut()) = 2_f64;
    (*m.entry(3).or_default().as_mut()) = 3_f64;
    let mut m_begin: UnsafeMapIterator<i32, f64> =
        UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<f64>>);
    let mut m_end: UnsafeMapIterator<i32, f64> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i32, Box<f64>>);
    let mut m_result_true: bool = m_begin != m_end;
    return ((((v_result_true) && (m_result_true))
        && ({
            let mut it = v.as_mut_ptr();
            while it != v.as_mut_ptr() && *it != 2 {
                it = it.add(1);
            }
            it
        } == v.as_mut_ptr())) as i32);
}
