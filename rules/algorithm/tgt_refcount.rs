// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f1<T1: Ord>(a0: Ptr<T1>, a1: Ptr<T1>) {
    a0.sort(a1.get_offset())
}

fn f2<T1: Clone + PartialOrd + ByteRepr, T2: Clone + From<T1> + ByteRepr>(
    a0: Ptr<T1>,
    a1: Ptr<T1>,
    a2: Ptr<T2>,
) -> Ptr<T2> {
    let count = a1.get_offset() - a0.get_offset();
    let mut outptr = a2.clone();
    for value in PtrValueIter::new(&a0, count) {
        outptr.write(value.into());
        outptr += 1;
    }
    outptr
}

fn f3<T1: PartialEq + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T1) -> Ptr<T1> {
    a0.offset(
        a0.clone()
            .into_iter()
            .enumerate()
            .position(|(index_0, value_0)| {
                index_0 < a1.get_offset() as usize && value_0.read() == a2
            })
            .unwrap_or(a1.get_offset() as usize) as isize,
    )
}

fn f6<T1: Ord + Clone, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: FnMut(Ptr<T1>, Ptr<T1>) -> bool,
{
    a0.sort_with_cmp(a1.get_offset(), a2)
}

fn f7<T1: Ord + Clone, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: FnMut(Ptr<T1>, Ptr<T1>) -> bool,
{
    a0.sort_with_cmp(a1.get_offset(), a2)
}

fn f8<T1: PartialOrd + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    let count = a1.get_offset() - a0.get_offset();
    let max_index = PtrValueIter::new(&a0, count)
        .enumerate()
        .max_by(|(_, val_a), (_, val_b)| {
            val_a
                .partial_cmp(val_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(idx, _)| idx)
        .unwrap_or(0);
    a0 + max_index
}

fn f9<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) {
    let tmp = a0.read();
    a0.write(a1.read());
    a1.write(tmp);
}

fn f10<T1: PartialEq + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    let count = a1.get_offset() - a0.get_offset();
    if count <= 1 {
        a1
    } else {
        let mut write_ptr = a0.clone();
        let mut iter = PtrValueIter::new(&a0, count);
        let mut last_unique = iter.next().unwrap();

        // the first unique value is already in place
        write_ptr += 1;

        for current_val in iter {
            if current_val != last_unique {
                write_ptr.write(current_val.clone());
                last_unique = current_val;
                write_ptr += 1;
            }
        }
        write_ptr
    }
}

fn f12<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T1) {
    let mut __a0 = a0.clone();
    while __a0 != a1 {
        let v = a2.clone();
        __a0.write(v);
        __a0 += 1;
    }
}

// TODO: a2 should be passed as &mut
fn f13(a0: Ptr<u8>, a1: Ptr<u8>, a2: &mut ::std::fs::File) -> ::std::fs::File {
    a2.write_all(
        a0.reinterpret_cast::<u8>()
            .slice_until(&a1.reinterpret_cast::<u8>())
            .as_slice(),
    );
    a2.try_clone().unwrap()
}

fn f14<T1: Ord + Clone + ByteRepr, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: Fn(T1, T1) -> bool,
{
    let fun = |x: Ptr<T1>, y: Ptr<T1>| a2((x.read()).clone(), (y.read()).clone());
    a0.sort_with_cmp(a1.get_offset(), fun)
}

fn f15<T1: Ord + Clone + ByteRepr, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: Fn(T1, T1) -> bool,
{
    let fun = |x: Ptr<T1>, y: Ptr<T1>| a2((x.read()).clone(), (y.read()).clone());
    a0.sort_with_cmp(a1.get_offset(), fun)
}

fn f16<T1: PartialOrd + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if a0.read() <= a1.read() { a0 } else { a1 }
}

fn f17<T1: PartialOrd + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    if a0.read() >= a1.read() { a0 } else { a1 }
}
