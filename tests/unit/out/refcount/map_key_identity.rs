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
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_1(
                Rc::new(RefCell::new(Key {
                    order: self.order.clone(),
                    identity: self.identity.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Key {
                    order: other.order.clone(),
                    identity: other.identity.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(
                Rc::new(RefCell::new(Key {
                    order: other.order.clone(),
                    identity: other.identity.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Key {
                    order: self.order.clone(),
                    identity: self.identity.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(Key {
                    order: self.order.clone(),
                    identity: self.identity.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Key {
                    order: other.order.clone(),
                    identity: other.identity.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Key {}
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
    let a: Ptr<Mapped> = (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with((*distinct_a.borrow()).clone(), || {
                (
                    Rc::new(RefCell::new((*distinct_a.borrow()).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
    field!(a, value).write(11);
    let a_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(
        (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
            .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&(*distinct_a.borrow()))),
    ));
    let mut a_value: Ptr<Mapped> = ((*(*a_it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    ok = { ((ok as i32) & ((default_constructions_0.with(|rc| *rc.borrow()) == 1) as i32)) != 0 };
    let b: Ptr<Mapped> = (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with((*distinct_b.borrow()).clone(), || {
                (
                    Rc::new(RefCell::new((*distinct_b.borrow()).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
    field!(b, value).write(22);
    let b_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(
        (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
            .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&(*distinct_b.borrow()))),
    ));
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
                == (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>).with(
                    |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&(*distinct_a.borrow())),
                )) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*b_it.borrow())
                == (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>).with(
                    |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&(*distinct_b.borrow())),
                )) as i32))
            != 0;
        ok = rhs_0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*b_it.borrow())
                != (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
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
                .with(|__s| (*__s.1.borrow()).value)
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
    let __begin1: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(
            (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.begin()),
        ));
    let __end1: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(
        (distinct.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
            .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let entry: Ptr<(Value<Key>, Value<Mapped>)> = (*__begin1.borrow()).iter_deref();
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
        (*__begin1.borrow_mut()).iter_inc();
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
    let first: Ptr<Mapped> = (equivalent.as_pointer()
        as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with((*equivalent_a.borrow()).clone(), || {
                (
                    Rc::new(RefCell::new((*equivalent_a.borrow()).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
    field!(first, value).write(31);
    let first_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(
            (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>).with(
                |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&(*equivalent_a.borrow())),
            ),
        ));
    let mut first_value: Ptr<Mapped> = ((*(*first_it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1)) as i32))
            != 0
    };
    let second: Ptr<Mapped> = (equivalent.as_pointer()
        as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with((*equivalent_b.borrow()).clone(), || {
                (
                    Rc::new(RefCell::new((*equivalent_b.borrow()).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
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
                    != (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()))
                    as i32))
                != 0;
            ok = rhs_0
        };
        ok = { ((ok as i32) & (({ (first).clone() } == { (second).clone() }) as i32)) != 0 };
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    == (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&(*equivalent_a.borrow()))
                        })) as i32))
                != 0;
            ok = rhs_0
        };
        {
            let rhs_0 = ((ok as i32)
                & (((*first_it.borrow())
                    == (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&(*equivalent_b.borrow()))
                        })) as i32))
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
                    .with(|__s| (*__s.1.borrow()).value)
                    == 31)
                    && (second.with(|__s| __s.value) == 31)) as i32))
                != 0;
            ok = rhs_0
        };
        let mut equivalent_count: usize = 0_usize;
        let __begin2: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
            Rc::new(RefCell::new(
                (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.begin()),
            ));
        let __end2: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
            Rc::new(RefCell::new(
                (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()),
            ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let entry: Ptr<(Value<Key>, Value<Mapped>)> = (*__begin2.borrow()).iter_deref();
            ok = {
                ((ok as i32)
                    & ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 3)
                        && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 31))
                        as i32))
                    != 0
            };
            equivalent_count.prefix_inc();
            (*__begin2.borrow_mut()).iter_inc();
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
    let mut later_key: Key = Key {
        order: 4,
        identity: 10,
    };
    let later: Ptr<Mapped> = (equivalent.as_pointer()
        as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with(later_key.clone(), || {
                (
                    Rc::new(RefCell::new((later_key).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
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
                    == (equivalent.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&(*equivalent_a.borrow()))
                        })) as i32))
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
                    .with(|__s| (*__s.1.borrow()).value)
                    == 31) as i32))
                != 0;
            ok = rhs_0
        };
    }
    let mut matching_a: Key = Key {
        order: 5,
        identity: 5,
    };
    let mut matching_b: Key = Key {
        order: 5,
        identity: 5,
    };
    let mut matching_c: Key = Key {
        order: 6,
        identity: 6,
    };
    let mut before_matching: i32 = default_constructions_0.with(|rc| *rc.borrow());
    let matching: Value<Map<Key, (Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(Map::new()));
    let matched: Ptr<Mapped> = (matching.as_pointer()
        as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with(matching_a.clone(), || {
                (
                    Rc::new(RefCell::new((matching_a).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
    field!(matched, value).write(51);
    let matched_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(
            (matching.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&matching_a)),
        ));
    let mut matched_value: Ptr<Mapped> = ((*(*matched_it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    ok = {
        ((ok as i32)
            & ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1)) as i32))
            != 0
    };
    let matched_again: Ptr<Mapped> = (matching.as_pointer()
        as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with(matching_b.clone(), || {
                (
                    Rc::new(RefCell::new((matching_b).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
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
                == (matching.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&matching_b)))
                as i32))
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
                .with(|__s| (*__s.1.borrow()).value)
                == 51) as i32))
            != 0;
        ok = rhs_0
    };
    let other: Ptr<Mapped> = (matching.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
        .with_mut(|__m: &mut Map<Key, (Value<Key>, Value<Mapped>)>| {
            __m.get_or_insert_with(matching_c.clone(), || {
                (
                    Rc::new(RefCell::new((matching_c).clone())),
                    Rc::new(RefCell::new(<Mapped>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        });
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
                == (matching.as_pointer() as Ptr<Map<Key, (Value<Key>, Value<Mapped>)>>)
                    .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&matching_a)))
                as i32))
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
                .with(|__s| (*__s.1.borrow()).value)
                == 51) as i32))
            != 0;
        ok = rhs_0
    };
    return if ok { 0 } else { 1 };
}
pub fn __cpp2rust_init_globals() {
    let _ = default_constructions_0.with(|_| ());
}
