extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..32).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct Tracked {
    #[offset(0)]
    pub id: i32,
}
impl Tracked {
    pub fn new_2() -> Self {
        Self { id: 0 }
    }
    pub fn new_1(mut id: i32) -> Self {
        Self { id: id }
    }
}
impl CppLess for Tracked {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Tracked {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_1(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_1(__rhs.clone(), __lhs.clone()))
    }
}
impl Default for Tracked {
    fn default() -> Self {
        { Tracked::new_2() }
    }
}
pub fn operator_lt_1(a: Ptr<Tracked>, b: Ptr<Tracked>) -> bool {
    return ({ a.with(|__s| __s.id) } < { b.with(|__s| __s.id) });
}
pub fn check_erase_2(mut key_destructor: bool, mut value_destructor: bool) {
    let one: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 1 })));
    let _dtor_one = ScopedDestructor::new(&one, |__p| __p.destructor());
    let two: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 2 })));
    let _dtor_two = ScopedDestructor::new(&two, |__p| __p.destructor());
    let three: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 3 })));
    let _dtor_three = ScopedDestructor::new(&three, |__p| __p.destructor());
    let m: Value<Map<Tracked, (Value<Tracked>, Value<Tracked>)>> =
        Rc::new(RefCell::new(Map::new()));
    Map::subscript(m.as_pointer(), one.as_pointer()).write(Tracked::new_1({ 11 }));
    Map::subscript(m.as_pointer(), two.as_pointer()).write(Tracked::new_1({ 12 }));
    Map::subscript(m.as_pointer(), three.as_pointer()).write(Tracked::new_1({ 13 }));
    'loop_: for n in IterRange::new(
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>),
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((32_i64) as isize),
    ) {
        let n: Ptr<i32> = (n).clone();
        n.write(0);
    }
    {
        let first: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m.find(&one.as_pointer()),
            )));
        let middle: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m.find(&two.as_pointer()),
            )));
        let last: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| {
                    __m.find(&three.as_pointer())
                },
            )));
        let retained: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new((*middle.borrow()).clone()));
        let retained_const: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new(<BidirectionalIterator<(
                Value<Tracked>,
                Value<Tracked>,
            )>>::from(
                ({ (*middle.borrow()).clone() })
            )));
        let mut first_value: Ptr<Tracked> = ((*(*first.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let mut last_value: Ptr<Tracked> = ((*(*last.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let end: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m.end(),
            )));
        let next: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new({
                let (__next, __entry) =
                    Map::erase(m.as_pointer(), &({ (*middle.borrow()).clone() }));
                let (__key, __value) = {
                    let __entry_ref = __entry.borrow();
                    (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
                };
                use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
                (&&Destroy(&__value)).destroy();
                (&&Destroy(&__key)).destroy();
                __next
            }));
        assert!(
            (({
                let __idx0 = (2) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (12) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (3) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (13) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            ((*m.borrow()).len() == 2_usize)
                && (m
                    .as_pointer()
                    .with(|__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m
                        .find(&two.as_pointer()))
                    == (*end.borrow()))
        );
        assert!(
            ((*next.borrow()) == (*last.borrow()))
                && ((*first.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m.begin()))
        );
        assert!(
            ({
                ({
                    ((*(*first.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (first_value).clone() })
                    && ({
                        ((*(*last.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer())
                    } == { (last_value).clone() })
            })
        );
        let walking: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> =
            Rc::new(RefCell::new((*first.borrow()).clone()));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*last.borrow()) }));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*end.borrow()) }));
        let const_first: Value<BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>> = Rc::new(
            RefCell::new(
                <BidirectionalIterator<(Value<Tracked>, Value<Tracked>)>>::from(
                    ({ (*first.borrow()).clone() }),
                ),
            ),
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) =
                Map::erase(m.as_pointer(), &({ (*const_first.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*m.borrow()).len() == 1_usize) && ((*next.borrow()) == (*last.borrow())))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m.begin())
                    == (*last.borrow()))
        );
        assert!(
            ({
                ({
                    ((*(*last.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (last_value).clone() })
            })
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) = Map::erase(m.as_pointer(), &({ (*last.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (3) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (13) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*next.borrow()) == (*end.borrow())) && ((*m.borrow()).len() == 0_usize))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<Tracked, (Value<Tracked>, Value<Tracked>)>| __m.begin())
                    == (*end.borrow()))
        );
    }
    let mut i: i32 = 0;
    'loop_: while (i < 32) {
        let mut expected: i32 = 0;
        if (i >= 1) && (i <= 3) {
            expected = (key_destructor as i32);
        }
        if (i >= 11) && (i <= 13) {
            expected = (value_destructor as i32);
        }
        assert!(
            (({
                let __idx0 = (i) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == expected)
        );
        i.prefix_inc();
    }
}
pub fn check_erase_3(mut key_destructor: bool, mut value_destructor: bool) {
    let one: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 1 })));
    let _dtor_one = ScopedDestructor::new(&one, |__p| __p.destructor());
    let two: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 2 })));
    let _dtor_two = ScopedDestructor::new(&two, |__p| __p.destructor());
    let three: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_1({ 3 })));
    let _dtor_three = ScopedDestructor::new(&three, |__p| __p.destructor());
    let m: Value<Map<Tracked, (Value<Tracked>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript(m.as_pointer(), one.as_pointer()).write(11);
    Map::subscript(m.as_pointer(), two.as_pointer()).write(12);
    Map::subscript(m.as_pointer(), three.as_pointer()).write(13);
    'loop_: for n in IterRange::new(
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>),
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((32_i64) as isize),
    ) {
        let n: Ptr<i32> = (n).clone();
        n.write(0);
    }
    {
        let first: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.find(&one.as_pointer()),
            )));
        let middle: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.find(&two.as_pointer()),
            )));
        let last: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.find(&three.as_pointer()),
            )));
        let retained: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new((*middle.borrow()).clone()));
        let retained_const: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> = Rc::new(
            RefCell::new(<BidirectionalIterator<(Value<Tracked>, Value<i32>)>>::from(
                ({ (*middle.borrow()).clone() }),
            )),
        );
        let mut first_value: Ptr<i32> = ((*(*first.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let mut last_value: Ptr<i32> = ((*(*last.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let end: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.end(),
            )));
        let next: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new({
                let (__next, __entry) =
                    Map::erase(m.as_pointer(), &({ (*middle.borrow()).clone() }));
                let (__key, __value) = {
                    let __entry_ref = __entry.borrow();
                    (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
                };
                use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
                (&&Destroy(&__value)).destroy();
                (&&Destroy(&__key)).destroy();
                __next
            }));
        assert!(
            (({
                let __idx0 = (2) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (12) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (3) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (13) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            ((*m.borrow()).len() == 2_usize)
                && (m
                    .as_pointer()
                    .with(|__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m
                        .find(&two.as_pointer()))
                    == (*end.borrow()))
        );
        assert!(
            ((*next.borrow()) == (*last.borrow()))
                && ((*first.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.begin()))
        );
        assert!(
            ({
                ({
                    ((*(*first.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (first_value).clone() })
                    && ({
                        ((*(*last.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer())
                    } == { (last_value).clone() })
            })
        );
        let walking: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> =
            Rc::new(RefCell::new((*first.borrow()).clone()));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*last.borrow()) }));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*end.borrow()) }));
        let const_first: Value<BidirectionalIterator<(Value<Tracked>, Value<i32>)>> = Rc::new(
            RefCell::new(<BidirectionalIterator<(Value<Tracked>, Value<i32>)>>::from(
                ({ (*first.borrow()).clone() }),
            )),
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) =
                Map::erase(m.as_pointer(), &({ (*const_first.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*m.borrow()).len() == 1_usize) && ((*next.borrow()) == (*last.borrow())))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.begin())
                    == (*last.borrow()))
        );
        assert!(
            ({
                ({
                    ((*(*last.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (last_value).clone() })
            })
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) = Map::erase(m.as_pointer(), &({ (*last.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (3) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (13) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*next.borrow()) == (*end.borrow())) && ((*m.borrow()).len() == 0_usize))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<Tracked, (Value<Tracked>, Value<i32>)>| __m.begin())
                    == (*end.borrow()))
        );
    }
    let mut i: i32 = 0;
    'loop_: while (i < 32) {
        let mut expected: i32 = 0;
        if (i >= 1) && (i <= 3) {
            expected = (key_destructor as i32);
        }
        if (i >= 11) && (i <= 13) {
            expected = (value_destructor as i32);
        }
        assert!(
            (({
                let __idx0 = (i) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == expected)
        );
        i.prefix_inc();
    }
}
pub fn check_erase_4(mut key_destructor: bool, mut value_destructor: bool) {
    let one: Value<i32> = Rc::new(RefCell::new(1));
    let two: Value<i32> = Rc::new(RefCell::new(2));
    let three: Value<i32> = Rc::new(RefCell::new(3));
    let m: Value<Map<i32, (Value<i32>, Value<Tracked>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript(m.as_pointer(), one.as_pointer()).write(Tracked::new_1({ 11 }));
    Map::subscript(m.as_pointer(), two.as_pointer()).write(Tracked::new_1({ 12 }));
    Map::subscript(m.as_pointer(), three.as_pointer()).write(Tracked::new_1({ 13 }));
    'loop_: for n in IterRange::new(
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>),
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((32_i64) as isize),
    ) {
        let n: Ptr<i32> = (n).clone();
        n.write(0);
    }
    {
        let first: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.find(&one.as_pointer()),
            )));
        let middle: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.find(&two.as_pointer()),
            )));
        let last: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.find(&three.as_pointer()),
            )));
        let retained: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new((*middle.borrow()).clone()));
        let retained_const: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> = Rc::new(
            RefCell::new(<BidirectionalIterator<(Value<i32>, Value<Tracked>)>>::from(
                ({ (*middle.borrow()).clone() }),
            )),
        );
        let mut first_value: Ptr<Tracked> = ((*(*first.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let mut last_value: Ptr<Tracked> = ((*(*last.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let end: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.end()),
            ));
        let next: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new({
                let (__next, __entry) =
                    Map::erase(m.as_pointer(), &({ (*middle.borrow()).clone() }));
                let (__key, __value) = {
                    let __entry_ref = __entry.borrow();
                    (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
                };
                use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
                (&&Destroy(&__value)).destroy();
                (&&Destroy(&__key)).destroy();
                __next
            }));
        assert!(
            (({
                let __idx0 = (2) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (12) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (3) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (13) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            ((*m.borrow()).len() == 2_usize)
                && (m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.find(&two.as_pointer())
                ) == (*end.borrow()))
        );
        assert!(
            ((*next.borrow()) == (*last.borrow()))
                && ((*first.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.begin()))
        );
        assert!(
            ({
                ({
                    ((*(*first.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (first_value).clone() })
                    && ({
                        ((*(*last.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer())
                    } == { (last_value).clone() })
            })
        );
        let walking: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> =
            Rc::new(RefCell::new((*first.borrow()).clone()));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*last.borrow()) }));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*end.borrow()) }));
        let const_first: Value<BidirectionalIterator<(Value<i32>, Value<Tracked>)>> = Rc::new(
            RefCell::new(<BidirectionalIterator<(Value<i32>, Value<Tracked>)>>::from(
                ({ (*first.borrow()).clone() }),
            )),
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) =
                Map::erase(m.as_pointer(), &({ (*const_first.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*m.borrow()).len() == 1_usize) && ((*next.borrow()) == (*last.borrow())))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.begin())
                    == (*last.borrow()))
        );
        assert!(
            ({
                ({
                    ((*(*last.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (last_value).clone() })
            })
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) = Map::erase(m.as_pointer(), &({ (*last.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (3) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (13) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*next.borrow()) == (*end.borrow())) && ((*m.borrow()).len() == 0_usize))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Tracked>)>| __m.begin())
                    == (*end.borrow()))
        );
    }
    let mut i: i32 = 0;
    'loop_: while (i < 32) {
        let mut expected: i32 = 0;
        if (i >= 1) && (i <= 3) {
            expected = (key_destructor as i32);
        }
        if (i >= 11) && (i <= 13) {
            expected = (value_destructor as i32);
        }
        assert!(
            (({
                let __idx0 = (i) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == expected)
        );
        i.prefix_inc();
    }
}
pub fn check_erase_5(mut key_destructor: bool, mut value_destructor: bool) {
    let one: Value<i32> = Rc::new(RefCell::new(1));
    let two: Value<i32> = Rc::new(RefCell::new(2));
    let three: Value<i32> = Rc::new(RefCell::new(3));
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript(m.as_pointer(), one.as_pointer()).write(11);
    Map::subscript(m.as_pointer(), two.as_pointer()).write(12);
    Map::subscript(m.as_pointer(), three.as_pointer()).write(13);
    'loop_: for n in IterRange::new(
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>),
        (destroyed_0.with(|v| v.as_pointer()) as Ptr<i32>).offset((32_i64) as isize),
    ) {
        let n: Ptr<i32> = (n).clone();
        n.write(0);
    }
    {
        let first: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&one.as_pointer())),
        ));
        let middle: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&two.as_pointer())),
        ));
        let last: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&three.as_pointer())),
        ));
        let retained: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new((*middle.borrow()).clone()));
        let retained_const: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(
            RefCell::new(<BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                ({ (*middle.borrow()).clone() }),
            )),
        );
        let mut first_value: Ptr<i32> = ((*(*first.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let mut last_value: Ptr<i32> = ((*(*last.borrow()).iter_deref().upgrade().deref())
            .1
            .as_pointer());
        let end: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
        ));
        let next: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new({
            let (__next, __entry) = Map::erase(m.as_pointer(), &({ (*middle.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        }));
        assert!(
            (({
                let __idx0 = (2) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (12) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (3) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 0)
                && (({
                    let __idx0 = (13) as usize;
                    destroyed_0.with(|rc| rc.borrow()[__idx0])
                }) == 0)
        );
        assert!(
            ((*m.borrow()).len() == 2_usize)
                && (m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&two.as_pointer()))
                    == (*end.borrow()))
        );
        assert!(
            ((*next.borrow()) == (*last.borrow()))
                && ((*first.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()))
        );
        assert!(
            ({
                ({
                    ((*(*first.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (first_value).clone() })
                    && ({
                        ((*(*last.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer())
                    } == { (last_value).clone() })
            })
        );
        let walking: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> =
            Rc::new(RefCell::new((*first.borrow()).clone()));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*last.borrow()) }));
        (*walking.borrow_mut()).iter_inc();
        assert!(({ (*walking.borrow()) == (*end.borrow()) }));
        let const_first: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(
            RefCell::new(<BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                ({ (*first.borrow()).clone() }),
            )),
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) =
                Map::erase(m.as_pointer(), &({ (*const_first.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (1) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (11) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*m.borrow()).len() == 1_usize) && ((*next.borrow()) == (*last.borrow())))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == (*last.borrow()))
        );
        assert!(
            ({
                ({
                    ((*(*last.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (last_value).clone() })
            })
        );
        (*next.borrow_mut()) = {
            let (__next, __entry) = Map::erase(m.as_pointer(), &({ (*last.borrow()).clone() }));
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        };
        assert!(
            (({
                let __idx0 = (3) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (key_destructor as i32))
        );
        assert!(
            (({
                let __idx0 = (13) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == (value_destructor as i32))
        );
        assert!(
            (((*next.borrow()) == (*end.borrow())) && ((*m.borrow()).len() == 0_usize))
                && (m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
                    == (*end.borrow()))
        );
    }
    let mut i: i32 = 0;
    'loop_: while (i < 32) {
        let mut expected: i32 = 0;
        if (i >= 1) && (i <= 3) {
            expected = (key_destructor as i32);
        }
        if (i >= 11) && (i <= 13) {
            expected = (value_destructor as i32);
        }
        assert!(
            (({
                let __idx0 = (i) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == expected)
        );
        i.prefix_inc();
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub id: i32,
}
impl MoveOnly {
    pub fn new(mut id: i32) -> Self {
        Self { id: id }
    }
    pub fn move_from(other: Ptr<MoveOnly>) -> Self {
        Self {
            id: other.with(|__s| __s.id),
        }
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_6(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_6(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_6(__rhs.clone(), __lhs.clone()))
    }
}
pub fn operator_lt_6(a: Ptr<MoveOnly>, b: Ptr<MoveOnly>) -> bool {
    return ({ a.with(|__s| __s.id) } < { b.with(|__s| __s.id) });
}
pub fn check_nested_move_only_7() {
    let outer: Value<
        Map<
            i32,
            (
                Value<i32>,
                Value<Map<MoveOnly, (Value<MoveOnly>, Value<Tracked>)>>,
            ),
        >,
    > = Rc::new(RefCell::new(Map::new()));
    let m: Ptr<Map<MoveOnly, (Value<MoveOnly>, Value<Tracked>)>> = Map::subscript_move(
        outer.as_pointer(),
        Rc::new(RefCell::new(0)).as_pointer(),
        |__k| cpp_move!(__k),
    );
    let key: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 4 })));
    let _dtor_key = ScopedDestructor::new(&key, |__p| __p.destructor());
    Map::subscript_move((m).clone(), key.as_pointer(), |__k| cpp_move!(__k))
        .write(Tracked::new_1({ 14 }));
    let __rhs = {
        (*destroyed_0.with(Value::clone).borrow_mut())[(14) as usize] = 0;
        ({
            let __idx0 = (14) as usize;
            destroyed_0.with(|rc| rc.borrow()[__idx0])
        })
    };
    (*destroyed_0.with(Value::clone).borrow_mut())[(4) as usize] = __rhs;
    let next: Value<BidirectionalIterator<(Value<MoveOnly>, Value<Tracked>)>> =
        Rc::new(RefCell::new({
            let (__next, __entry) = Map::erase(
                (m).clone(),
                &m.with(|__m: &Map<MoveOnly, (Value<MoveOnly>, Value<Tracked>)>| __m.begin()),
            );
            let (__key, __value) = {
                let __entry_ref = __entry.borrow();
                (__entry_ref.0.as_pointer(), __entry_ref.1.as_pointer())
            };
            use libcc2rs::__swap::{Destroy, DestroyNone, DestroyWithDtor};
            (&&Destroy(&__value)).destroy();
            (&&Destroy(&__key)).destroy();
            __next
        }));
    assert!(
        (({
            let __idx0 = (4) as usize;
            destroyed_0.with(|rc| rc.borrow()[__idx0])
        }) == 1)
            && (({
                let __idx0 = (14) as usize;
                destroyed_0.with(|rc| rc.borrow()[__idx0])
            }) == 1)
    );
    assert!(
        ((*m.upgrade().deref()).len() == 0_usize)
            && ((*next.borrow())
                == m.with(|__m: &Map<MoveOnly, (Value<MoveOnly>, Value<Tracked>)>| __m.end()))
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ check_erase_2(true, true) });
    ({ check_erase_3(true, false) });
    ({ check_erase_4(false, true) });
    ({ check_erase_5(false, false) });
    ({ check_nested_move_only_7() });
    0
}
pub trait MoveOnlyImpl {
    fn destructor(&self);
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut())[((*self).with(|__s| __s.id)) as usize]
            .prefix_inc();
    }
}
pub trait TrackedImpl {
    fn destructor(&self);
}
impl TrackedImpl for Ptr<Tracked> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut())[((*self).with(|__s| __s.id)) as usize]
            .prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
}
