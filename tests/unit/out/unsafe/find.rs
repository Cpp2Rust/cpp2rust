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
    let mut v_begin: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
    let mut v_end: UnsafeRandomAccessIterator<i32> =
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
    let mut it: UnsafeRandomAccessIterator<i32> = {
        let mut __it = v_begin.clone();
        while __it != v_end.clone() && *__it.iter_deref() != 2 {
            __it.iter_inc();
        }
        __it
    };
    let mut v_result_true: bool =
        it != UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
    let mut m: BTreeMap<i32, Box<(i32, f64)>> = BTreeMap::new();
    (*&mut m
        .entry(1.clone())
        .or_insert_with(|| Box::new((1, <f64>::default())))
        .1) = 1_f64;
    (*&mut m
        .entry(2.clone())
        .or_insert_with(|| Box::new((2, <f64>::default())))
        .1) = 2_f64;
    (*&mut m
        .entry(3.clone())
        .or_insert_with(|| Box::new((3, <f64>::default())))
        .1) = 3_f64;
    let mut m_begin: UnsafeBidirectionalIterator<(i32, f64)> =
        UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<(i32, f64)>>);
    let mut m_end: UnsafeBidirectionalIterator<(i32, f64)> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i32, Box<(i32, f64)>>);
    let mut m_result_true: bool = m_begin != m_end;
    assert!(
        ((v_result_true) && (m_result_true))
            && ({
                let mut __it = UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
                while __it != UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())
                    && *__it.iter_deref() != 2
                {
                    __it.iter_inc();
                }
                __it
            } == UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
    );
    let mut w: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert!(
        {
            let mut __it = UnsafeRandomAccessIterator::Vec(w.as_mut_ptr()).iter_add(2_isize);
            while __it != UnsafeRandomAccessIterator::Vec(w.as_mut_ptr().add(w.len()))
                && *__it.iter_deref() != 4
            {
                __it.iter_inc();
            }
            __it
        } == UnsafeRandomAccessIterator::Vec(w.as_mut_ptr()).iter_add(3_isize)
    );
    assert!(
        {
            let mut __it = UnsafeRandomAccessIterator::Vec(w.as_mut_ptr()).iter_add(2_isize);
            while __it != UnsafeRandomAccessIterator::Vec(w.as_mut_ptr().add(w.len()))
                && *__it.iter_deref() != 1
            {
                __it.iter_inc();
            }
            __it
        } == UnsafeRandomAccessIterator::Vec(w.as_mut_ptr().add(w.len()))
    );
    assert!(
        {
            let mut __it = UnsafeRandomAccessIterator::Vec(w.as_mut_ptr()).iter_add(2_isize);
            while __it != UnsafeRandomAccessIterator::Vec(w.as_mut_ptr()).iter_add(4_isize)
                && *__it.iter_deref() != 5
            {
                __it.iter_inc();
            }
            __it
        } == UnsafeRandomAccessIterator::Vec(w.as_mut_ptr()).iter_add(4_isize)
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
