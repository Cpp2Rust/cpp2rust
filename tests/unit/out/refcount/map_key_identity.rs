extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static default_constructions_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Key {
    #[offset(0)]
    pub order: i32,
    #[offset(4)]
    pub identity: i32,
}
impl CppLess for Key {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_1(a: Ptr<Key>, b: Ptr<Key>) -> bool {
    return ({ a.with(|__s| __s.order) } < { b.with(|__s| __s.order) });
}
pub fn operator_eq_2(a: Ptr<Key>, b: Ptr<Key>) -> bool {
    return ({ a.with(|__s| __s.identity) } == { b.with(|__s| __s.identity) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Mapped {
    #[offset(0)]
    pub value: i32,
}
impl Mapped {
    pub fn new() -> Self {
        let __this: Mapped = Self { value: 0 };
        (*default_constructions_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for Mapped {
    fn default() -> Self {
        { Mapped::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut ok: bool = true;
    let distinct_a: Value<Key> = Rc::new(RefCell::new(Key {
        order: 1,
        identity: 7,
    }));
    let distinct_b: Value<Key> = Rc::new(RefCell::new(Key {
        order: 2,
        identity: 7,
    }));
    {
        let rhs_0 = ((ok as i32)
            & (({
                let _a: Ptr<Key> = distinct_a.as_pointer();
                operator_eq_2(_a, distinct_b.as_pointer())
            }) as i32))
            != 0;
        ok = rhs_0
    };
    let distinct: Value<Map<Key, (Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(Map::new()));
    let a: Ptr<Mapped> = Map::subscript(distinct.as_pointer(), distinct_a.as_pointer());
    field!(a, value).write(11);
    let a_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(distinct.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&distinct_a.as_pointer()),
        )));
    let mut a_value: Ptr<Mapped> = ((*(*a_it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    ok = { ((ok as i32) & ((default_constructions_0.with(|rc| *rc.borrow()) == 1) as i32)) != 0 };
    let b: Ptr<Mapped> = Map::subscript(distinct.as_pointer(), distinct_b.as_pointer());
    field!(b, value).write(22);
    let b_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(distinct.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&distinct_b.as_pointer()),
        )));
    ok = { ((ok as i32) & ((default_constructions_0.with(|rc| *rc.borrow()) == 2) as i32)) != 0 };
    {
        let rhs_0 = ((ok as i32) & (((*distinct.borrow()).len() == 2_usize) as i32)) != 0;
        ok = rhs_0
    };
    ok = { ((ok as i32) & (({ (a).clone() } != { (b).clone() }) as i32)) != 0 };
    ok = {
        ((ok as i32)
            & (((a.with(|__s| __s.value) == 11) && (b.with(|__s| __s.value) == 22)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*a_it.borrow())
                == distinct
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                        __m.find(&distinct_a.as_pointer())
                    })) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*b_it.borrow())
                == distinct
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                        __m.find(&distinct_b.as_pointer())
                    })) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*b_it.borrow())
                != distinct
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()))
                as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (({
                ((*(*a_it.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer())
            } == { (a_value).clone() }) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*a_it.borrow())
                .iter_deref()
                .with(|__s| (*({ __s }).1.borrow()).value)
                == 11) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (({
                let _a: Ptr<Key> = distinct_a.as_pointer();
                operator_eq_2(_a, distinct_b.as_pointer())
            }) as i32))
            != 0;
        ok = rhs_0
    };
    let mut distinct_count: usize = 0_usize;
    'loop_: for entry in IterRange::new(
        distinct
            .as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.begin()),
        distinct
            .as_pointer()
            .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()),
    ) {
        let entry: Ptr<(Value<Key>, Value<Mapped>)> = entry.iter_deref();
        if (distinct_count == 0_usize) {
            ok = {
                ((ok as i32)
                    & ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 1)
                        && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 11))
                        as i32))
                    != 0
            };
        } else if (distinct_count == 1_usize) {
            ok = {
                ((ok as i32)
                    & ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 2)
                        && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 22))
                        as i32))
                    != 0
            };
        }
        distinct_count.prefix_inc();
    }
    ok = { ((ok as i32) & ((distinct_count == 2_usize) as i32)) != 0 };
    let equivalent_a: Value<Key> = Rc::new(RefCell::new(Key {
        order: 3,
        identity: 8,
    }));
    let equivalent_b: Value<Key> = Rc::new(RefCell::new(Key {
        order: 3,
        identity: 9,
    }));
    {
        let rhs_0 = ((ok as i32)
            & (!({
                let _a: Ptr<Key> = equivalent_a.as_pointer();
                operator_eq_2(_a, equivalent_b.as_pointer())
            }) as i32))
            != 0;
        ok = rhs_0
    };
    let equivalent: Value<Map<Key, (Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(Map::new()));
    let mut before_equivalent: i32 = default_constructions_0.with(|rc| *rc.borrow());
    let first: Ptr<Mapped> = Map::subscript(equivalent.as_pointer(), equivalent_a.as_pointer());
    field!(first, value).write(31);
    let first_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(equivalent.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&equivalent_a.as_pointer()),
        )));
    let mut first_value: Ptr<Mapped> = ((*(*first_it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1)) as i32))
            != 0
    };
    let second: Ptr<Mapped> = Map::subscript(equivalent.as_pointer(), equivalent_b.as_pointer());
    let mut reused: bool =
        (default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1));
    ok = { ((ok as i32) & (reused as i32)) != 0 };
    {
        let rhs_0 = ((ok as i32) & (((*equivalent.borrow()).len() == 1_usize) as i32)) != 0;
        ok = rhs_0
    };
    if reused {
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    != equivalent
                        .as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()))
                    as i32))
                != 0;
            ok = rhs_0
        };
        ok = { ((ok as i32) & (({ (first).clone() } == { (second).clone() }) as i32)) != 0 };
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    == equivalent.as_pointer().with(
                        |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&equivalent_a.as_pointer())
                        },
                    )) as i32))
                != 0;
            ok = rhs_0
        };
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    == equivalent.as_pointer().with(
                        |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&equivalent_b.as_pointer())
                        },
                    )) as i32))
                != 0;
            ok = rhs_0
        };
        {
            let rhs_0 = ((ok as i32)
                & (({
                    ((*(*first_it.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (first_value).clone() }) as i32))
                != 0;
            ok = rhs_0
        };
        {
            let rhs_0 = ((ok as i32)
                & ((((*first_it.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 31)
                    && (second.with(|__s| __s.value) == 31)) as i32))
                != 0;
            ok = rhs_0
        };
        let mut equivalent_count: usize = 0_usize;
        'loop_: for entry in IterRange::new(
            equivalent
                .as_pointer()
                .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.begin()),
            equivalent
                .as_pointer()
                .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()),
        ) {
            let entry: Ptr<(Value<Key>, Value<Mapped>)> = entry.iter_deref();
            ok = {
                ((ok as i32)
                    & ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 3)
                        && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 31))
                        as i32))
                    != 0
            };
            equivalent_count.prefix_inc();
        }
        ok = { ((ok as i32) & ((equivalent_count == 1_usize) as i32)) != 0 };
    }
    {
        let rhs_0 = ((ok as i32)
            & (!({
                let _a: Ptr<Key> = equivalent_a.as_pointer();
                operator_eq_2(_a, equivalent_b.as_pointer())
            }) as i32))
            != 0;
        ok = rhs_0
    };
    let later_key: Value<Key> = Rc::new(RefCell::new(Key {
        order: 4,
        identity: 10,
    }));
    let later: Ptr<Mapped> = Map::subscript(equivalent.as_pointer(), later_key.as_pointer());
    field!(later, value).write(41);
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 2)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32) & (((*equivalent.borrow()).len() == 2_usize) as i32)) != 0;
        ok = rhs_0
    };
    if reused {
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    == equivalent.as_pointer().with(
                        |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&equivalent_a.as_pointer())
                        },
                    )) as i32))
                != 0;
            ok = rhs_0
        };
        {
            let rhs_0 = ((ok as i32)
                & (({
                    ((*(*first_it.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer())
                } == { (first_value).clone() }) as i32))
                != 0;
            ok = rhs_0
        };
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 31) as i32))
                != 0;
            ok = rhs_0
        };
    }
    let matching_a: Value<Key> = Rc::new(RefCell::new(Key {
        order: 5,
        identity: 5,
    }));
    let matching_b: Value<Key> = Rc::new(RefCell::new(Key {
        order: 5,
        identity: 5,
    }));
    let matching_c: Value<Key> = Rc::new(RefCell::new(Key {
        order: 6,
        identity: 6,
    }));
    let mut before_matching: i32 = default_constructions_0.with(|rc| *rc.borrow());
    let matching: Value<Map<Key, (Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(Map::new()));
    let matched: Ptr<Mapped> = Map::subscript(matching.as_pointer(), matching_a.as_pointer());
    field!(matched, value).write(51);
    let matched_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(matching.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&matching_a.as_pointer()),
        )));
    let mut matched_value: Ptr<Mapped> = ((*(*matched_it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1)) as i32))
            != 0
    };
    let matched_again: Ptr<Mapped> = Map::subscript(matching.as_pointer(), matching_b.as_pointer());
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32) & (((*matching.borrow()).len() == 1_usize) as i32)) != 0;
        ok = rhs_0
    };
    ok = { ((ok as i32) & (({ (matched).clone() } == { (matched_again).clone() }) as i32)) != 0 };
    {
        let rhs_0 = ((ok as i32)
            & (((*matched_it.borrow())
                == matching
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                        __m.find(&matching_b.as_pointer())
                    })) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (({
                ((*(*matched_it.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer())
            } == { (matched_value).clone() }) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*matched_it.borrow())
                .iter_deref()
                .with(|__s| (*({ __s }).1.borrow()).value)
                == 51) as i32))
            != 0;
        ok = rhs_0
    };
    let other: Ptr<Mapped> = Map::subscript(matching.as_pointer(), matching_c.as_pointer());
    field!(other, value).write(61);
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 2)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32) & (((*matching.borrow()).len() == 2_usize) as i32)) != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*matched_it.borrow())
                == matching
                    .as_pointer()
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                        __m.find(&matching_a.as_pointer())
                    })) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (({
                ((*(*matched_it.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer())
            } == { (matched_value).clone() }) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*matched_it.borrow())
                .iter_deref()
                .with(|__s| (*({ __s }).1.borrow()).value)
                == 51) as i32))
            != 0;
        ok = rhs_0
    };
    return if ok { 0 } else { 1 };
}
pub fn __cpp2rust_init_globals() {
    let _ = default_constructions_0.with(|_| ());
}
