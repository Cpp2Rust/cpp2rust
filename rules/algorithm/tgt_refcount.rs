// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f1<T1: Ord + 'static>(a0: Ptr<T1>, a1: Ptr<T1>) {
    a0.sort(a1.get_offset())
}

fn f2<T1: Clone + PartialOrd + ByteRepr, T2: Clone + From<T1> + ByteRepr>(
    a0: Ptr<T1>,
    a1: Ptr<T1>,
    a2: Ptr<T2>,
) -> Ptr<T2> {
    let count = a1.get_offset() - a0.get_offset();
    let mut outptr = a2;
    for value in PtrValueIter::new(&a0, count) {
        outptr.write(value.into());
        outptr += 1;
    }
    outptr
}

fn f3<T1: PartialEq + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T1) -> Ptr<T1> {
    let count = (a1.get_offset() - a0.get_offset()) as usize;
    a0.offset(
        a0.clone()
            .into_iter()
            .take(count)
            .position(|value_0| value_0.read() == a2)
            .unwrap_or(count) as isize,
    )
}

fn f6<T1: Ord + Clone + 'static, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: Callable2<Ptr<T1>, Ptr<T1>, bool>,
{
    a0.sort_with_cmp(a1.get_offset(), |x, y| a2.call(x, y))
}

fn f8<T1: PartialOrd + Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    let count = a1.get_offset() - a0.get_offset();
    let max_index = PtrValueIter::new(&a0, count)
        .enumerate()
        .max_by(|(idx_a, val_a), (idx_b, val_b)| {
            val_a
                .partial_cmp(val_b)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| idx_b.cmp(idx_a))
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
