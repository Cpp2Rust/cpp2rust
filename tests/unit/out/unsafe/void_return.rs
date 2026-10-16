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
        let __n = last.clone().iter_distance(&first.clone());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order
            .sort_by(|&__x, &__y| (*first.clone().iter_at(__x)).cmp(&*first.clone().iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(first.clone().iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(first.clone().iter_at(__i), __v);
        }
    };
    return;
}
pub unsafe fn f1_1(mut first: *mut i32, mut last: *mut i32) {
    {
        let __n = last.iter_distance(&first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*first.iter_at(__x)).cmp(&*first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(first.iter_at(__i), __v);
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
    let mut v: Vec<i32> = vec![1, 3, 0, 2, 8, 7];
    (unsafe {
        let _first: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr());
        let _last: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(v.len()));
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
