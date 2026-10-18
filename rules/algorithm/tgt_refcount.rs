// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f1<T1: CppRandomAccessIterator<T2>, T2: Ord + Clone + ByteRepr>(a0: T1, a1: T1) {
    let __n = a1.iter_distance(&a0);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        a0.iter_at(__x)
            .with(|__a| a0.iter_at(__y).with(|__b| __a.cmp(__b)))
    });
    let __values: Vec<_> = __order.iter().map(|&__i| a0.iter_at(__i).read()).collect();
    for (__i, __v) in (0..__n).zip(__values) {
        a0.iter_at(__i).write(__v);
    }
}

fn f2<
    T1: CppInputIterator<T3>,
    T2: CppIterator<T4>,
    T3: Clone + ByteRepr,
    T4: From<T3> + ByteRepr,
>(
    a0: T1,
    a1: T1,
    a2: T2,
) -> T2 {
    let mut __first = a0;
    let mut __out = a2;
    while __first != a1 {
        __out.iter_deref().write(__first.iter_deref().read().into());
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}

fn f3<T1: CppInputIterator<T3>, T2, T3: PartialEq<T2> + ByteRepr>(a0: T1, a1: T1, a2: T2) -> T1 {
    let mut __it = a0;
    while __it != a1 && __it.iter_deref().with(|__v| *__v != a2) {
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
    let __values: Vec<_> = __order.iter().map(|&__i| a0.iter_at(__i).read()).collect();
    for (__i, __v) in (0..__n).zip(__values) {
        a0.iter_at(__i).write(__v);
    }
}

fn f8<T1: CppInputIterator<T2>, T2: PartialOrd + ByteRepr>(a0: T1, a1: T1) -> T1 {
    let mut __best = a0.clone();
    if a0 != a1 {
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            if __best
                .iter_deref()
                .with(|__b| __it.iter_deref().with(|__v| __b < __v))
            {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

fn f9<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) {
    let tmp = a0.read();
    a0.write(a1.read());
    a1.write(tmp);
}

fn f10<T1: CppInputIterator<T2>, T2: PartialEq + Clone + ByteRepr>(a0: T1, a1: T1) -> T1 {
    if a0 == a1 {
        a1
    } else {
        let mut __write = a0.clone();
        let mut __it = a0;
        __it.iter_inc();
        while __it != a1 {
            let __v = __it.iter_deref().read();
            if __write.iter_deref().with(|__w| *__w != __v) {
                __write.iter_inc();
                __write.iter_deref().write(__v);
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

fn f13(a0: Ptr<i8>, a1: Ptr<i8>, a2: &mut ::std::fs::File) -> ::std::fs::File {
    a0.with_slice(a1.get_offset() - a0.get_offset(), |__s| {
        CChar::with_u8_slice(__s, |__b| a2.write_all(__b))
    });
    a2.try_clone().unwrap()
}

fn f14<T1: Ord + Clone + ByteRepr, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: Callable2<T1, T1, bool>,
{
    let fun = |x: Ptr<T1>, y: Ptr<T1>| a2.call((x.read()).clone(), (y.read()).clone());
    a0.sort_with_cmp(a1.get_offset(), fun)
}

fn f16<T1: PartialOrd + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if a0.read() <= a1.read() { a0 } else { a1 }
}

fn f17<T1: PartialOrd + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if a0.read() >= a1.read() { a0 } else { a1 }
}

fn f18<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) {
    let count = a1.get_offset() - a0.get_offset();
    let random = std::hash::RandomState::new();
    for i in (1..count).rev() {
        let j = std::hash::BuildHasher::hash_one(&random, i) as usize % (i + 1);
        let x = a0.offset(i);
        let y = a0.offset(j);
        let tmp = x.read();
        x.write(y.read());
        y.write(tmp);
    }
}
