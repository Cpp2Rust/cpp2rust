// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t2<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t3<T1>() -> Vec<Value<Vec<T1>>> {
    Vec::new()
}

fn t4<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f1<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: Ptr<T1>) -> Ptr<T1> {
    let idx = a1.get_offset();
    a0.with_mut(|__v: &mut Vec<T1>| __v.remove(idx));
    a0.to_strong().as_pointer() as Ptr<T1>
}

fn f6<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f7<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f9<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f10<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f13<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f17<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f18<T1: ByteRepr>(a0: &mut Vec<T1>, a1: Ptr<T1>, a2: T1) -> Ptr<T1> {
    let __off = a1.get_offset();
    a0.insert(__off, a2);
    a1
}

fn f20<T1: ByteRepr>(a0: &mut Vec<T1>, a1: Ptr<T1>, a2: T1) -> Ptr<T1> {
    let __off = a1.get_offset();
    a0.insert(__off, a2);
    a1
}

fn f22<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f23<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.clone()
}

fn f24<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.clone()
}

fn f25<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f26<T1>(a0: Ptr<T1>, a1: Ptr<T1>) -> bool {
    a0 != a1
}

fn f27<T1>(a0: Ptr<T1>, a1: Ptr<T1>) -> bool {
    a0 == a1
}

fn f28<T1>(a0: &mut Ptr<T1>) -> Ptr<T1> {
    a0.postfix_inc()
}

fn f29<T1: Clone>(a0: Vec<Value<Vec<T1>>>) -> Vec<Value<Vec<T1>>> {
    a0.iter()
        .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
        .collect()
}

fn f30<T1: Default + Clone>(a0: usize) -> Vec<Value<Vec<T1>>> {
    (0..(a0) as usize)
        .map(|_| <Value<Vec<T1>>>::default())
        .collect::<Vec<_>>()
}

fn f31<T1: ByteRepr + Clone>(a0: Ptr<Vec<Value<Vec<T1>>>>, a1: Vec<T1>) {
    a0.with_mut(|__v: &mut Vec<Value<Vec<T1>>>| __v.push(Rc::new(RefCell::new(a1.clone()))))
}

fn f32<T1: Default + ByteRepr>(a0: Ptr<Vec<Value<Vec<T1>>>>, a1: usize) {
    let _a0 = a1 as usize;
    a0.with_mut(|__v: &mut Vec<Value<Vec<T1>>>| __v.resize_with(_a0, <Value<Vec<T1>>>::default))
}

fn f33<T1>(a0: Ptr<T1>, a1: Ptr<T1>) -> isize {
    ((a0.get_offset() as isize) - (a1.get_offset() as isize))
}

fn f34<T1>(a0: &mut Ptr<T1>) -> Ptr<T1> {
    a0.prefix_inc()
}

fn f35<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Vec<T1> {
    let __count = a1.get_offset() - a0.get_offset();
    PtrValueIter::new(&a0, __count).collect::<Vec<_>>()
}

fn f37<T1: TryFrom<T2>, T2: Clone + ByteRepr>(a0: Ptr<T2>, a1: Ptr<T2>) -> Vec<T1> {
    let __count = a1.get_offset() - a0.get_offset();
    PtrValueIter::new(&a0, __count)
        .map(|item| T1::try_from(item).ok().unwrap())
        .collect::<Vec<_>>()
}

fn f40<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f41<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f42(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    let __count = a1.get_offset() - a0.get_offset();
    let max_index = PtrValueIter::new(&a0, __count)
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    a0 + max_index
}

fn f43<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f44<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f50<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<Vec<T1>> {
    if a1 as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.offset(a1 as isize)
    }
}

fn f51<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f52<T1: Clone + ByteRepr>(a0: Ptr<Vec<Value<Vec<T1>>>>, a1: Vec<T1>) {
    a0.with_mut(|__v: &mut Vec<Value<Vec<T1>>>| __v.push(Rc::new(RefCell::new(a1.clone()))))
}

fn f53<T1: Clone + ByteRepr>(
    a0: Ptr<Vec<T1>>,
    a1: Ptr<T1>,
    a2: Ptr<T1>,
    a3: Ptr<T1>,
) -> Ptr<Vec<T1>> {
    let start_idx = a1.get_offset();
    let count = a3.get_offset() - a2.get_offset();
    let temp_vec: Vec<T1> = PtrValueIter::new(&a2, count).collect();
    a0.with_mut(|v: &mut Vec<T1>| {
        v.splice(start_idx..start_idx, temp_vec);
    });
    a0 + start_idx
}

fn f56<T1: ByteRepr>(a0: &Vec<Value<Vec<T1>>>) -> Ptr<Vec<T1>> {
    a0[a0.len() - 1].as_pointer()
}

fn f55<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f57<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f58<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1.clone())
}
