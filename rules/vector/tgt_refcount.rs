// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f40<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f60<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: RandomAccessIterator<T1>) -> RandomAccessIterator<T1> {
    let idx = a1.vec_ptr().get_offset();
    a0.with_mut(|__v: &mut Vec<T1>| __v.remove(idx));
    RandomAccessIterator::Vec(a0.decay())
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
    let __off = a1.vec_ptr().get_offset();
    a0.insert(__off, a2);
    a1
}

fn f79<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __off = a1.vec_ptr().get_offset();
    a0.insert(__off, a2);
    a1
}

fn f90<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Vec<T1> {
    let __count = a1.get_offset() - a0.get_offset();
    PtrValueIter::new(&a0, __count).collect::<Vec<_>>()
}

fn f92<T1: TryFrom<T2>, T2: Clone + ByteRepr>(a0: Ptr<T2>, a1: Ptr<T2>) -> Vec<T1> {
    let __count = a1.get_offset() - a0.get_offset();
    PtrValueIter::new(&a0, __count)
        .map(|item| T1::try_from(item).ok().unwrap())
        .collect::<Vec<_>>()
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

fn f98<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<Vec<T1>> {
    if a1 as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.offset(a1 as isize)
    }
}

fn f99<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f101<T1: Clone + ByteRepr>(
    a0: Ptr<Vec<T1>>,
    a1: RandomAccessIterator<T1>,
    a2: Ptr<T1>,
    a3: Ptr<T1>,
) -> RandomAccessIterator<T1> {
    let start_idx = a1.vec_ptr().get_offset();
    let count = a3.get_offset() - a2.get_offset();
    let temp_vec: Vec<T1> = PtrValueIter::new(&a2, count).collect();
    a0.with_mut(|v: &mut Vec<T1>| {
        v.splice(start_idx..start_idx, temp_vec);
    });
    RandomAccessIterator::Vec(a0.decay() + start_idx)
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
