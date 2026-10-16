// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1<T1, T2>() -> Map<T1, (Value<T1>, Value<T2>)> {
    Map::new()
}

fn f1<T1: Ord + Clone + ByteRepr + 'static, T2: Default + ByteRepr + 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: T1,
) -> Ptr<T2> {
    a0.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| {
        __m.get_or_insert_with(a1.clone(), || {
            (
                Rc::new(RefCell::new(a1)),
                Rc::new(RefCell::new(<T2>::default())),
            )
        })
        .borrow()
        .1
        .as_pointer()
    })
}

fn f2<T1, T2>(a0: Map<T1, (Value<T1>, Value<T2>)>) -> usize {
    a0.len()
}

fn f3<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: BidirectionalIterator<(Value<T1>, Value<T2>)>,
) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| __m.erase(&a1))
}

fn f5<T1, T2>() -> Map<T1, (Value<T1>, Value<T2>)> {
    Map::new()
}

fn f6<T1: Ord + Clone, T2: Clone>(a0: Map<T1, (Value<T1>, Value<T2>)>) -> Map<T1, (Value<T1>, Value<T2>)> {
    let mut __m = Map::new();
    for (__k, __v) in a0.entries() {
        let __v = __v.borrow();
        __m.insert(
            __k.clone(),
            Rc::new(RefCell::new((
                Rc::new(RefCell::new(__v.0.borrow().clone())),
                Rc::new(RefCell::new(__v.1.borrow().clone())),
            ))),
        );
    }
    __m
}

fn f7<T1: Ord, T2>(a0: &mut Map<T1, (Value<T1>, Value<T2>)>, a1: T1) -> Ptr<T2> {
    a0.get(&a1).expect("out of range!").borrow().1.as_pointer()
}

fn f8<T1: Ord + Clone + ByteRepr + 'static, T2: Default + ByteRepr + 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: T1,
) -> Ptr<T2> {
    a0.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| {
        __m.get_or_insert_with(a1.clone(), || {
            (
                Rc::new(RefCell::new(a1)),
                Rc::new(RefCell::new(<T2>::default())),
            )
        })
        .borrow()
        .1
        .as_pointer()
    })
}

fn f9<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.end())
}

fn f10<T1: Ord + 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: T1) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.find(&a1))
}

fn f12<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.begin())
}

fn f14<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.end())
}

fn f15<T1: Ord, T2>(a0: &mut Map<T1, (Value<T1>, Value<T2>)>, a1: T1) -> Ptr<T2> {
    a0.get(&a1).expect("out of range!").borrow().1.as_pointer()
}

fn f17<T1: Ord + 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: T1) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.find(&a1))
}

fn f24<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> Map<T1, (Value<T1>, Value<T2>)> {
    a0.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| std::mem::take(__m))
}

fn f25<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) {
    let __src = a1.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| std::mem::take(__m));
    a0.write(__src)
}

fn f26<T1: Ord + Clone + 'static, T2: Clone + 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Map<T1, (Value<T1>, Value<T2>)>) {
    let mut __m = Map::new();
    for (__k, __v) in a1.entries() {
        let __v = __v.borrow();
        __m.insert(
            __k.clone(),
            Rc::new(RefCell::new((
                Rc::new(RefCell::new(__v.0.borrow().clone())),
                Rc::new(RefCell::new(__v.1.borrow().clone())),
            ))),
        );
    }
    a0.write(__m)
}
