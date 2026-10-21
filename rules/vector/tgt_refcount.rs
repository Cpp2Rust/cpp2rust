// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t13<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t14<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f40<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f60<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: RandomAccessIterator<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.with_mut(|__v: &mut Vec<T1>| __v.remove(__off));
    __it
}

fn f65<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f66<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f68<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f69<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f72<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f76<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f77<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f79<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f92<T1: TryFrom<T2>, T2: Clone + ByteRepr, T3: CppInputIterator<T2>>(a0: T3, a1: T3) -> Vec<T1> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push(<T1>::try_from(__it.iter_deref().read()).ok().unwrap());
        __it.iter_inc();
    }
    __v
}

fn f93<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f95<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f96<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f98<T1: ByteRepr + 'static>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    let __idx = a1;
    if __idx as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(__idx as isize)
    }
}

fn f99<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f101<T1: Clone + ByteRepr, T2, T3: CppInputIterator<T1>>(
    a0: Ptr<Vec<T1>>,
    a1: RandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> RandomAccessIterator<T1> {
    let __pos = a1;
    let start_idx = __pos.index();
    let mut temp_vec: Vec<T1> = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        temp_vec.push(__it.iter_deref().read());
        __it.iter_inc();
    }
    a0.with_mut(|v: &mut Vec<T1>| {
        v.splice(start_idx..start_idx, temp_vec);
    });
    __pos
}

fn f103<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f104<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f105<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f115<T1: Clone + ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: usize, a3: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    __it
}

fn f116<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: Vec<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, a2);
    __it
}
