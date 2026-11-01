// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f1<T1: CppRandomAccessIterator<T2>, T2: CppLess + Clone + ByteRepr>(a0: T1, a1: T1) {
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| cpp_cmp(&a0.iter_at(__x), &a0.iter_at(__y)));
    reorder(__order, |__i| a0.iter_at(__i));
}

fn f2<
    T1: CppInputIterator<T3>,
    T2: CppIterator<T4>,
    T3: ByteRepr,
    T4: CppFrom<T3> + ByteRepr,
>(
    a0: T1,
    a1: T1,
    a2: T2,
) -> T2 {
    let mut __first = a0;
    let mut __out = a2;
    while __first != a1 {
        __out.iter_deref().write(T4::cpp_from(__first.iter_deref()));
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}

fn f3<T1: CppInputIterator<T3>, T2, T3: CppEq<T2>>(a0: T1, a1: T1, a2: Ptr<T2>) -> T1 {
    let mut __it = a0;
    while __it != a1 && !CppEq::equal(&__it.iter_deref(), &a2) {
        __it.iter_inc();
    }
    __it
}

fn f6<T1: CppRandomAccessIterator<T3>, T2, T3: Clone + ByteRepr>(a0: T1, a1: T1, a2: T2)
where
    T2: Callable2<Ptr<T3>, Ptr<T3>, bool>,
{
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        let __a = a0.iter_at(__x);
        let __b = a0.iter_at(__y);
        if a2.call(__a.clone(), __b.clone()) {
            std::cmp::Ordering::Less
        } else if a2.call(__b, __a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    reorder(__order, |__i| a0.iter_at(__i));
}

fn f8<T1: CppInputIterator<T2>, T2: CppLess>(a0: T1, a1: T1) -> T1 {
    let mut __best = a0.clone();
    if a0 != a1 {
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            if CppLess::less(&__best.iter_deref(), &__it.iter_deref()) {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

fn f9<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) {
    cpp_swap!(a0, a1)
}

fn f10<T1: CppInputIterator<T2>, T2: CppEq + Clone + ByteRepr>(a0: T1, a1: T1) -> T1 {
    if a0 == a1 {
        a1
    } else {
        let mut __write = a0.clone();
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                __write.iter_inc();
                __write.iter_deref().write(__it.iter_deref().read());
            }
            __it.iter_inc();
        }
        __write.iter_inc();
        __write
    }
}

fn f12<T1: CppInputIterator<T2>, T2: Clone + ByteRepr>(a0: T1, a1: T1, a2: T2) {
    let mut __it = a0;
    while __it != a1 {
        __it.iter_deref().write(Clone::clone(&a2));
        __it.iter_inc();
    }
}

fn f13<T1: CppInputIterator<i8>>(
    a0: T1,
    a1: T1,
    a2: &mut libcc2rs::CppStream,
) -> libcc2rs::CppStream {
    let mut __it = a0;
    let __out = &mut *a2;
    let mut __bytes: Vec<u8> = Vec::new();
    while __it != a1 {
        __bytes.push(__it.iter_deref().read() as u8);
        __it.iter_inc();
    }
    __out.write_all(&__bytes);
    __out.try_clone().unwrap()
}

fn f14<T1: Clone + ByteRepr, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: Callable2<T1, T1, bool>,
{
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        let __a = a0.iter_at(__x);
        let __b = a0.iter_at(__y);
        if a2.call(__a.read(), __b.read()) {
            std::cmp::Ordering::Less
        } else if a2.call(__b.read(), __a.read()) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    reorder(__order, |__i| a0.iter_at(__i));
}

fn f16<T1: CppLess>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if CppLess::less(&a1, &a0) { a1 } else { a0 }
}

fn f17<T1: CppLess>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if CppLess::less(&a0, &a1) { a1 } else { a0 }
}

fn f18<T1: CppRandomAccessIterator<T2>, T2: Clone + ByteRepr>(a0: T1, a1: T1) {
    let __n = a1.iter_distance(&a0);
    let __random = std::hash::RandomState::new();
    for __i in (1..__n).rev() {
        let __j = (std::hash::BuildHasher::hash_one(&__random, __i) % (__i + 1) as u64) as isize;
        cpp_swap!(a0.iter_at(__i), a0.iter_at(__j));
    }
}
