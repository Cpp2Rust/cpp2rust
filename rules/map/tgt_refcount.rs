// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

fn t1<T1, T2>() -> BTreeMap<T1, Value<(Value<T1>, Value<T2>)>> {
    BTreeMap::new()
}

fn f1<T1: Ord + Clone + ByteRepr + 'static, T2: Default + ByteRepr + 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: T1,
) -> Ptr<T2> {
    a0.with_mut(|__v: &mut BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>| {
        __v.entry(a1.clone())
            .or_insert_with(|| {
                Rc::new(RefCell::new((
                    Rc::new(RefCell::new(a1)),
                    Rc::new(RefCell::new(<T2>::default())),
                )))
            })
            .borrow()
            .1
            .as_pointer()
    })
}

fn f2<T1, T2>(a0: BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>) -> usize {
    a0.len()
}

fn f3<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: RefcountMapIter<T1, T2>,
) -> RefcountMapIter<T1, T2> {
    RefcountMapIter::erase(a0, &a1)
}

fn f5<T1, T2>() -> BTreeMap<T1, Value<(Value<T1>, Value<T2>)>> {
    BTreeMap::new()
}

fn f6<T1: Ord + Clone, T2: Clone>(
    a0: BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>,
) -> BTreeMap<T1, Value<(Value<T1>, Value<T2>)>> {
    a0.iter()
        .map(|(k, v)| {
            let __v = v.borrow();
            (
                k.clone(),
                Rc::new(RefCell::new((
                    Rc::new(RefCell::new(__v.0.borrow().clone())),
                    Rc::new(RefCell::new(__v.1.borrow().clone())),
                ))),
            )
        })
        .collect()
}

fn f7<T1: Ord, T2: Default>(
    a0: &mut BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>,
    a1: T1,
) -> Ptr<T2> {
    a0.get(&a1).expect("out of range!").borrow().1.as_pointer()
}

fn f8<T1: Ord + Clone + ByteRepr + 'static, T2: Default + ByteRepr + 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: T1,
) -> Ptr<T2> {
    a0.with_mut(|__v: &mut BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>| {
        __v.entry(a1.clone())
            .or_insert_with(|| {
                Rc::new(RefCell::new((
                    Rc::new(RefCell::new(a1)),
                    Rc::new(RefCell::new(<T2>::default())),
                )))
            })
            .borrow()
            .1
            .as_pointer()
    })
}

fn f9<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
) -> RefcountMapIter<T1, T2> {
    RefcountMapIter::end(a0)
}

fn f10<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: T1,
) -> RefcountMapIter<T1, T2> {
    RefcountMapIter::find_key(a0, &a1)
}

fn f12<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
) -> RefcountMapIter<T1, T2> {
    RefcountMapIter::begin(a0)
}

fn f14<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
) -> RefcountMapIter<T1, T2> {
    RefcountMapIter::end(a0)
}

fn f15<T1: Ord, T2: Default>(
    a0: &mut BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>,
    a1: T1,
) -> Ptr<T2> {
    a0.get(&a1).expect("out of range!").borrow().1.as_pointer()
}

fn f17<T1: Ord + Clone + 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: T1,
) -> RefcountMapIter<T1, T2> {
    RefcountMapIter::find_key(a0, &a1)
}

fn f24<T1: 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
) -> BTreeMap<T1, Value<(Value<T1>, Value<T2>)>> {
    a0.with_mut(|__v: &mut BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>| std::mem::take(__v))
}

fn f25<T1: 'static, T2: 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
) {
    let __src =
        a1.with_mut(|__v: &mut BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>| std::mem::take(__v));
    a0.write(__src)
}

fn f26<T1: Ord + Clone + 'static, T2: Clone + 'static>(
    a0: Ptr<BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>>,
    a1: BTreeMap<T1, Value<(Value<T1>, Value<T2>)>>,
) {
    a0.write(
        a1.iter()
            .map(|(k, v)| {
                let __v = v.borrow();
                (
                    k.clone(),
                    Rc::new(RefCell::new((
                        Rc::new(RefCell::new(__v.0.borrow().clone())),
                        Rc::new(RefCell::new(__v.1.borrow().clone())),
                    ))),
                )
            })
            .collect(),
    )
}

