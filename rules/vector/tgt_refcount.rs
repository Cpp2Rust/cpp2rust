// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

struct T1;

fn types() {
    let t2: Ptr<T1> = Ptr::null();
    let t3: Vec<Value<Vec<T1>>> = Vec::new();
    let t4: Ptr<T1> = Ptr::null();
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
    let mut __a0 = a0.clone();
    let mut __out = Vec::with_capacity(a1.get_offset() - __a0.get_offset());
    while __a0 != a1 {
        __out.push(__a0.read());
        __a0 += 1;
    }
    __out
}

fn f37<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Vec<T1> {
    let mut __a0 = a0.clone();
    let mut __out = Vec::with_capacity(a1.get_offset() - __a0.get_offset());
    while __a0 != a1 {
        __out.push(__a0.read());
        __a0 += 1;
    }
    __out
}

fn f39(a0: Ptr<u32>, a1: Ptr<u32>) -> Vec<i32> {
    let mut __a0 = a0.clone();
    let mut __out = Vec::with_capacity(a1.get_offset() - __a0.get_offset());
    while __a0 != a1 {
        __out.push(__a0.read() as i32);
        __a0 += 1;
    }
    __out
}

fn f40<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_end()
}

fn f41<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f42(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<u8> {
    if a0 == a1 {
        a0.clone()
    } else {
        let mut __a0 = a0.clone();
        let mut max_it = a0.clone();
        __a0 += 1;

        while __a0 != a1 {
            if max_it.read() < __a0.read() {
                max_it = __a0.clone();
            }
            __a0 += 1;
        }
        max_it
    }
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
    let mut __idx = a1.get_offset() as usize;
    let mut __a2 = a2.clone();
    while __a2 != a3 {
        a0.with_mut(|__v: &mut Vec<T1>| __v.insert(__idx, __a2.read()));
        __idx += 1;
        __a2 += 1;
    }
    a0
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
