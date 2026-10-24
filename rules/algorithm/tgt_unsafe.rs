// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::{Seek, Write};

unsafe fn f1<T1: UnsafeCppRandomAccessIterator<T2>, T2: Ord>(a0: T1, a1: T1) {
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(__first.iter_at(__i)))
        .collect();
    for (__i, __v) in (0..__n).zip(__values) {
        std::ptr::write(__first.iter_at(__i), __v);
    }
}

unsafe fn f2<T1: UnsafeCppInputIterator<T3>, T2: UnsafeCppIterator<T4>, T3: Clone, T4: From<T3>>(
    a0: T1,
    a1: T1,
    a2: T2,
) -> T2 {
    let mut __first = a0;
    let __last = a1;
    let mut __out = a2;
    while __first != __last {
        *__out.iter_deref() = (*__first.iter_deref()).clone().into();
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}

unsafe fn f3<T1: UnsafeCppInputIterator<T3>, T2, T3: PartialEq<T2>>(a0: T1, a1: T1, a2: T2) -> T1 {
    let mut __it = a0;
    let __last = a1;
    let __value = &a2;
    while __it != __last && *__it.iter_deref() != *__value {
        __it.iter_inc();
    }
    __it
}

unsafe fn f6<T1: UnsafeCppRandomAccessIterator<T3>, T2, T3>(a0: T1, a1: T1, a2: T2)
where
    T2: Callable2<*const T3, *const T3, bool>,
{
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    let __cmp = a2;
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        let __a = __first.iter_at(__x) as *const _;
        let __b = __first.iter_at(__y) as *const _;
        if __cmp.call(__a, __b) {
            std::cmp::Ordering::Less
        } else if __cmp.call(__b, __a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(__first.iter_at(__i)))
        .collect();
    for (__i, __v) in (0..__n).zip(__values) {
        std::ptr::write(__first.iter_at(__i), __v);
    }
}

unsafe fn f8<T1: UnsafeCppInputIterator<T2>, T2: PartialOrd>(a0: T1, a1: T1) -> T1 {
    let __first = a0;
    let __last = a1;
    let mut __best = __first.clone();
    if __first != __last {
        let mut __it = __first;
        __it.iter_inc();
        while __it != __last {
            if *__best.iter_deref() < *__it.iter_deref() {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

unsafe fn f9<T1>(a0: *mut T1, a1: *mut T1) {
    cpp_swap_unsafe!(a0, a1)
}

unsafe fn f10<T1: UnsafeCppInputIterator<T2>, T2: PartialEq + Clone>(a0: T1, a1: T1) -> T1 {
    let __first = a0;
    let __last = a1;
    if __first == __last {
        __last
    } else {
        let mut __write = __first.clone();
        let mut __it = __first;
        __it.iter_inc();
        while __it != __last {
            if *__write.iter_deref() != *__it.iter_deref() {
                __write.iter_inc();
                *__write.iter_deref() = (*__it.iter_deref()).clone();
            }
            __it.iter_inc();
        }
        __write.iter_inc();
        __write
    }
}

unsafe fn f12<T1: UnsafeCppInputIterator<T2>, T2: Clone>(a0: T1, a1: T1, a2: T2) {
    let mut __it = a0;
    let __last = a1;
    let __value = &a2;
    while __it != __last {
        *__it.iter_deref() = Clone::clone(__value);
        __it.iter_inc();
    }
}

unsafe fn f13<T1: UnsafeCppInputIterator<libc::c_char>>(
    a0: T1,
    a1: T1,
    a2: &mut ::std::fs::File,
) -> ::std::fs::File {
    let mut __it = a0;
    let __last = a1;
    let __out = &mut *a2;
    let mut __bytes: Vec<u8> = Vec::new();
    while __it != __last {
        __bytes.push(*__it.iter_deref() as u8);
        __it.iter_inc();
    }
    __out.write_all(&__bytes);
    __out.try_clone().unwrap()
}

unsafe fn f14<T1: Ord + Copy, T2>(a0: *mut T1, a1: *mut T1, a2: T2)
where
    T2: Callable2<T1, T1, bool>,
{
    let __first = a0;
    let len = a1.offset_from(__first) as usize;
    let __cmp = a2;
    ::std::slice::from_raw_parts_mut(__first, len).sort_by(|x, y| {
        if __cmp.call(*x, *y) {
            std::cmp::Ordering::Less
        } else if __cmp.call(*y, *x) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    })
}

unsafe fn f16<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    let __a: *const _ = a0;
    let __b: *const _ = a1;
    if *__b < *__a { __b } else { __a }
}

unsafe fn f17<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    let __a: *const _ = a0;
    let __b: *const _ = a1;
    if *__a < *__b { __b } else { __a }
}

unsafe fn f18<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) {
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    for __i in (1..__n).rev() {
        let __j = libc::rand() as isize % (__i + 1);
        cpp_swap_unsafe!(__first.iter_at(__i), __first.iter_at(__j));
    }
}
