extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f1_0(
    mut first: UnsafeRandomAccessIterator<i32>,
    mut last: UnsafeRandomAccessIterator<i32>,
) {
    {
        let __first = (first).clone();
        let __n = (last).clone().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    return;
}
pub unsafe fn f1_1(mut first: *mut i32, mut last: *mut i32) {
    {
        let __first = first;
        let __n = last.iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::from([1, 3, 0, 2, 8, 7]);
    (unsafe {
        let _first: UnsafeRandomAccessIterator<i32> = v.begin();
        let _last: UnsafeRandomAccessIterator<i32> = v.end();
        f1_0(_first, _last)
    });
    let mut a: [i32; 3] = [1, 2, 3];
    (unsafe {
        let _first: *mut i32 = a.as_mut_ptr();
        let _last: *mut i32 = a.as_mut_ptr().offset((2) as isize);
        f1_1(_first, _last)
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
