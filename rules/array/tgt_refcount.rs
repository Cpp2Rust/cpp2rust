// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1<T1: Default, const T2: usize>() -> Box<[T1]> {
    Box::<[T1]>::from(std::array::from_fn::<_, T2, _>(|_| <T1>::default()))
}

fn t4<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t5<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f1<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f3<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f4<T1: MoveCtor, const T2: usize>(a0: Ptr<T1>) -> Box<[T1]> {
    (0..T2).map(|__i| T1::move_ctor(a0.offset(__i))).collect()
}

fn f5<T1: MoveAssign, const T2: usize>(a0: Ptr<T1>, a1: Ptr<T1>) {
    for __i in 0..T2 {
        T1::move_assign(&a0.offset(__i), a1.offset(__i));
    }
}

fn f7<T1: CopyAssign, const T2: usize>(a0: Ptr<T1>, a1: Ptr<T1>) {
    for __i in 0..T2 {
        T1::copy_assign(&a0.offset(__i), a1.offset(__i));
    }
}

fn f8<T1, const T2: usize>(a0: [T1; T2]) -> Box<[T1]> {
    Box::<[T1]>::from(a0)
}

fn f9<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}

fn f10<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}

fn f11<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}

fn f12<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}
