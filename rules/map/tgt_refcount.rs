// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1, T2>() -> Map<T1, (Value<T1>, Value<T2>)> {
    Map::new()
}

fn f1<T1: CppLess + Clone + ByteRepr + 'static, T2: Default + 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: Ptr<T1>,
) -> Ptr<T2> {
    Map::subscript(a0, a1)
}

fn f2<T1, T2>(a0: Map<T1, (Value<T1>, Value<T2>)>) -> usize {
    a0.len()
}

fn f3<T1: CppLess + 'static, T2: 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: BidirectionalIterator<(Value<T1>, Value<T2>)>,
) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    let __it = a1;
    Map::erase(a0, &__it)
}

fn f5<T1, T2>() -> Map<T1, (Value<T1>, Value<T2>)> {
    Map::new()
}

fn f6<T1: CppLess + Clone + 'static, T2: Clone>(a0: Map<T1, (Value<T1>, Value<T2>)>) -> Map<T1, (Value<T1>, Value<T2>)> {
    a0.clone()
}

fn f7<T1: CppLess + 'static, T2>(a0: &mut Map<T1, (Value<T1>, Value<T2>)>, a1: Ptr<T1>) -> Ptr<T2> {
    a0.get(&a1).expect("out of range!").borrow().1.as_pointer()
}

fn f8<T1: CppLess + 'static, T2: Default + 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: Ptr<T1>,
) -> Ptr<T2> {
    Map::subscript_move(a0, a1, |__k| cpp_move!(__k))
}

fn f9<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.end())
}

fn f10<T1: CppLess + 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Ptr<T1>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.find(&a1))
}

fn f12<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.begin())
}

fn f14<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.end())
}

fn f15<T1: CppLess + 'static, T2>(a0: &mut Map<T1, (Value<T1>, Value<T2>)>, a1: Ptr<T1>) -> Ptr<T2> {
    a0.get(&a1).expect("out of range!").borrow().1.as_pointer()
}

fn f17<T1: CppLess + 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Ptr<T1>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.find(&a1))
}

fn f24<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> Map<T1, (Value<T1>, Value<T2>)> {
    a0.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| std::mem::take(__m))
}

fn f25<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) {
    let __src = a1.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| std::mem::take(__m));
    a0.write(__src)
}

fn f26<T1: CppLess + Clone + 'static, T2: Clone + 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Map<T1, (Value<T1>, Value<T2>)>) {
    a0.write(a1.clone())
}

fn f27<T1: CppLess + 'static, T2: 'static>(
    a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>,
    a1: BidirectionalIterator<(Value<T1>, Value<T2>)>,
) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    let __it = a1;
    Map::erase(a0, &__it)
}

fn f28<T1: CppLess + 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Ptr<T1>) -> usize {
    Map::erase_key(a0, &a1)
}

fn f29<T1: 'static, T2: 'static>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) -> BidirectionalIterator<(Value<T1>, Value<T2>)> {
    a0.with(|__m: &Map<T1, (Value<T1>, Value<T2>)>| __m.begin())
}
