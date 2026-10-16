// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::{Seek, Write};

unsafe fn f1<T1: UnsafeCppRandomAccessIterator<T2>, T2: Ord>(a0: T1, a1: T1) {
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| (*a0.iter_at(__x)).cmp(&*a0.iter_at(__y)));
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(a0.iter_at(__i)))
        .collect();
    for (__i, __v) in __values.into_iter().enumerate() {
        std::ptr::write(a0.iter_at(__i as isize), __v);
    }
}

unsafe fn f2<T1: UnsafeCppInputIterator<T3>, T2: UnsafeCppIterator<T4>, T3: Clone, T4: From<T3>>(
    a0: T1,
    a1: T1,
    a2: T2,
) -> T2 {
    let mut __first = a0;
    let mut __out = a2;
    while __first != a1 {
        *__out.iter_deref() = (*__first.iter_deref()).clone().into();
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}

unsafe fn f3<T1: UnsafeCppInputIterator<T3>, T2, T3: PartialEq<T2>>(a0: T1, a1: T1, a2: T2) -> T1 {
    let mut __it = a0;
    while __it != a1 && *__it.iter_deref() != a2 {
        __it.iter_inc();
    }
    __it
}

unsafe fn f6<T1: UnsafeCppRandomAccessIterator<T3>, T2, T3>(a0: T1, a1: T1, a2: T2)
where
    T2: Callable2<*const T3, *const T3, bool>,
{
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        let __a = a0.iter_at(__x) as *const _;
        let __b = a0.iter_at(__y) as *const _;
        if a2.call(__a, __b) {
            std::cmp::Ordering::Less
        } else if a2.call(__b, __a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(a0.iter_at(__i)))
        .collect();
    for (__i, __v) in __values.into_iter().enumerate() {
        std::ptr::write(a0.iter_at(__i as isize), __v);
    }
}

unsafe fn f8<T1: UnsafeCppInputIterator<T2>, T2: PartialOrd>(a0: T1, a1: T1) -> T1 {
    let mut __best = a0.clone();
    if a0 != a1 {
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            if *__best.iter_deref() < *__it.iter_deref() {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

unsafe fn f9<T1>(a0: &mut T1, a1: &mut T1) {
    std::mem::swap(&mut *a0, &mut *a1)
}

unsafe fn f13(
    a0: *const libc::c_char,
    a1: *const libc::c_char,
    a2: &mut ::std::fs::File,
) -> ::std::fs::File {
    let __start = a0 as *const u8;
    let __end = a1 as *const u8;
    let __len = __end.offset_from(__start) as usize;
    a2.write_all(::std::slice::from_raw_parts(__start, __len));
    a2.try_clone().unwrap()
}

unsafe fn f14<T1: Ord + Copy, T2>(a0: *mut T1, a1: *mut T1, a2: T2)
where
    T2: Callable2<T1, T1, bool>,
{
    let len = a1.offset_from(a0) as usize;
    ::std::slice::from_raw_parts_mut(a0, len).sort_by(|x, y| {
        if a2.call(*x, *y) {
            std::cmp::Ordering::Less
        } else if a2.call(*y, *x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

unsafe fn f16<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    if *a0 <= *a1 {
        (a0) as *const _
    } else {
        (a1) as *const _
    }
}

unsafe fn f17<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    if *a0 >= *a1 {
        (a0) as *const _
    } else {
        (a1) as *const _
    }
}
