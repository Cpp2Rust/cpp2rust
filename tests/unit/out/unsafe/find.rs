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
    let mut v_begin: UnsafeRandomAccessIterator<i32> = v.begin();
    let mut v_end: UnsafeRandomAccessIterator<i32> = v.end();
    let mut it: UnsafeRandomAccessIterator<i32> = match ({ (v_end).clone() },) {
        (mut __tmp_0,) => {
            let mut __it = (v_begin).clone();
            while __it != __tmp_0 && *__it.iter_deref() != 2 {
                __it.iter_inc();
            }
            __it
        }
    };
    let mut v_result_true: bool = it != v.end();
    let mut m: UnsafeMap<i32, (i32, f64)> = UnsafeMap::new();
    (*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) = 1_f64;
    (*m.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))) = 2_f64;
    (*m.subscript_move(&mut 3, |__k| cpp_move_unsafe!(__k))) = 3_f64;
    let mut m_begin: UnsafeBidirectionalIterator<(i32, f64)> = m.begin();
    let mut m_end: UnsafeBidirectionalIterator<(i32, f64)> = m.end();
    let mut m_result_true: bool = m_begin != m_end;
    assert!(
        ((v_result_true) && (m_result_true))
            && (match ({ v.begin() }, { v.begin() },) {
                (mut __tmp_1, mut __tmp_2) => {
                    {
                        let mut __it = (__tmp_1).clone();
                        while __it != __tmp_2 && *__it.iter_deref() != 2 {
                            __it.iter_inc();
                        }
                        __it
                    }
                }
            } == v.begin())
    );
    let mut w: Vec<i32> = Vec::from([1, 2, 3, 4, 5]);
    assert!(
        match ({ w.begin().iter_add(2_isize) }, { w.end() },) {
            (mut __tmp_3, mut __tmp_4) => {
                {
                    let mut __it = (__tmp_3).clone();
                    while __it != __tmp_4 && *__it.iter_deref() != 4 {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        } == w.begin().iter_add(3_isize)
    );
    assert!(
        match ({ w.begin().iter_add(2_isize) }, { w.end() },) {
            (mut __tmp_5, mut __tmp_6) => {
                {
                    let mut __it = (__tmp_5).clone();
                    while __it != __tmp_6 && *__it.iter_deref() != 1 {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        } == w.end()
    );
    assert!(
        match ({ w.begin().iter_add(2_isize) }, {
            w.begin().iter_add(4_isize)
        },)
        {
            (mut __tmp_7, mut __tmp_8) => {
                {
                    let mut __it = (__tmp_7).clone();
                    while __it != __tmp_8 && *__it.iter_deref() != 5 {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        } == w.begin().iter_add(4_isize)
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
