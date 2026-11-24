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
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
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
    libcc2rs::cpp2rust_exit(main_0());
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
        match (
            {
                (({
                    let _a: Ptr<Key> = distinct_a.as_pointer();
                    operator_eq_2(_a, distinct_b.as_pointer())
                }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_0, __tmp_1) => {
                (*__tmp_1) = (((*__tmp_1) as i32) & (__tmp_0 as i32)) != 0;
            }
        }
    };
    let distinct: Value<Map<Key, (Value<Key>, Value<Mapped>)>> = Rc::new(RefCell::new(Map::new()));
    let a: Ptr<Mapped> =
        ({ (*distinct.borrow_mut()).subscript(distinct_a.as_pointer(), || <Mapped>::default()) });
    field!(a, value).write(11);
    let a_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(distinct.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&distinct_a.as_pointer()),
        )));
    let mut a_value: Ptr<Mapped> = ((*({ (*a_it.borrow()).iter_deref() }).upgrade().deref())
        .1
        .as_pointer());
    {
        match (
            { ((default_constructions_0.with(|rc| *rc.borrow()) == 1) as i32) },
            &mut (ok),
        ) {
            (__tmp_2, __tmp_3) => {
                (*__tmp_3) = (((*__tmp_3) as i32) & (__tmp_2 as i32)) != 0;
            }
        }
    };
    let b: Ptr<Mapped> =
        ({ (*distinct.borrow_mut()).subscript(distinct_b.as_pointer(), || <Mapped>::default()) });
    field!(b, value).write(22);
    let b_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(distinct.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&distinct_b.as_pointer()),
        )));
    {
        match (
            { ((default_constructions_0.with(|rc| *rc.borrow()) == 2) as i32) },
            &mut (ok),
        ) {
            (__tmp_4, __tmp_5) => {
                (*__tmp_5) = (((*__tmp_5) as i32) & (__tmp_4 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (((*distinct.borrow()).len() == 2_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_6, __tmp_7) => {
                (*__tmp_7) = (((*__tmp_7) as i32) & (__tmp_6 as i32)) != 0;
            }
        }
    };
    {
        match ({ (({ (a).clone() } != { (b).clone() }) as i32) }, &mut (ok)) {
            (__tmp_8, __tmp_9) => {
                (*__tmp_9) = (((*__tmp_9) as i32) & (__tmp_8 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (((a.with(|__s| __s.value) == 11) && (b.with(|__s| __s.value) == 22)) as i32) },
            &mut (ok),
        ) {
            (__tmp_10, __tmp_11) => {
                (*__tmp_11) = (((*__tmp_11) as i32) & (__tmp_10 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((*a_it.borrow())
                    == distinct
                        .as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&distinct_a.as_pointer())
                        })) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_12, __tmp_13) => {
                (*__tmp_13) = (((*__tmp_13) as i32) & (__tmp_12 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((*b_it.borrow())
                    == distinct
                        .as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&distinct_b.as_pointer())
                        })) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_14, __tmp_15) => {
                (*__tmp_15) = (((*__tmp_15) as i32) & (__tmp_14 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((*b_it.borrow())
                    != distinct
                        .as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()))
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_16, __tmp_17) => {
                (*__tmp_17) = (((*__tmp_17) as i32) & (__tmp_16 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (({
                    ((*({ (*a_it.borrow()).iter_deref() }).upgrade().deref())
                        .1
                        .as_pointer())
                } == { (a_value).clone() }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_18, __tmp_19) => {
                (*__tmp_19) = (((*__tmp_19) as i32) & (__tmp_18 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                ((({ (*a_it.borrow()).iter_deref() }).with(|__s| (*__s.1.borrow()).value) == 11)
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_20, __tmp_21) => {
                (*__tmp_21) = (((*__tmp_21) as i32) & (__tmp_20 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (({
                    let _a: Ptr<Key> = distinct_a.as_pointer();
                    operator_eq_2(_a, distinct_b.as_pointer())
                }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_22, __tmp_23) => {
                (*__tmp_23) = (((*__tmp_23) as i32) & (__tmp_22 as i32)) != 0;
            }
        }
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
            {
                match (
                    {
                        ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 1)
                            && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 11))
                            as i32)
                    },
                    &mut (ok),
                ) {
                    (__tmp_24, __tmp_25) => {
                        (*__tmp_25) = (((*__tmp_25) as i32) & (__tmp_24 as i32)) != 0;
                    }
                }
            };
        } else if (distinct_count == 1_usize) {
            {
                match (
                    {
                        ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 2)
                            && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 22))
                            as i32)
                    },
                    &mut (ok),
                ) {
                    (__tmp_26, __tmp_27) => {
                        (*__tmp_27) = (((*__tmp_27) as i32) & (__tmp_26 as i32)) != 0;
                    }
                }
            };
        }
        distinct_count.prefix_inc();
    }
    {
        match ({ ((distinct_count == 2_usize) as i32) }, &mut (ok)) {
            (__tmp_28, __tmp_29) => {
                (*__tmp_29) = (((*__tmp_29) as i32) & (__tmp_28 as i32)) != 0;
            }
        }
    };
    let equivalent_a: Value<Key> = Rc::new(RefCell::new(Key {
        order: 3,
        identity: 8,
    }));
    let equivalent_b: Value<Key> = Rc::new(RefCell::new(Key {
        order: 3,
        identity: 9,
    }));
    {
        match (
            {
                (!({
                    let _a: Ptr<Key> = equivalent_a.as_pointer();
                    operator_eq_2(_a, equivalent_b.as_pointer())
                }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_30, __tmp_31) => {
                (*__tmp_31) = (((*__tmp_31) as i32) & (__tmp_30 as i32)) != 0;
            }
        }
    };
    let equivalent: Value<Map<Key, (Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(Map::new()));
    let mut before_equivalent: i32 = default_constructions_0.with(|rc| *rc.borrow());
    let first: Ptr<Mapped> = ({
        (*equivalent.borrow_mut()).subscript(equivalent_a.as_pointer(), || <Mapped>::default())
    });
    field!(first, value).write(31);
    let first_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(equivalent.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&equivalent_a.as_pointer()),
        )));
    let mut first_value: Ptr<Mapped> =
        ((*({ (*first_it.borrow()).iter_deref() }).upgrade().deref())
            .1
            .as_pointer());
    {
        match (
            {
                ((default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1))
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_32, __tmp_33) => {
                (*__tmp_33) = (((*__tmp_33) as i32) & (__tmp_32 as i32)) != 0;
            }
        }
    };
    let second: Ptr<Mapped> = ({
        (*equivalent.borrow_mut()).subscript(equivalent_b.as_pointer(), || <Mapped>::default())
    });
    let mut reused: bool =
        (default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 1));
    {
        match ({ (reused as i32) }, &mut (ok)) {
            (__tmp_34, __tmp_35) => {
                (*__tmp_35) = (((*__tmp_35) as i32) & (__tmp_34 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (((*equivalent.borrow()).len() == 1_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_36, __tmp_37) => {
                (*__tmp_37) = (((*__tmp_37) as i32) & (__tmp_36 as i32)) != 0;
            }
        }
    };
    if reused {
        {
            match (
                {
                    (((*first_it.borrow())
                        != equivalent
                            .as_pointer()
                            .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.end()))
                        as i32)
                },
                &mut (ok),
            ) {
                (__tmp_38, __tmp_39) => {
                    (*__tmp_39) = (((*__tmp_39) as i32) & (__tmp_38 as i32)) != 0;
                }
            }
        };
        {
            match (
                { (({ (first).clone() } == { (second).clone() }) as i32) },
                &mut (ok),
            ) {
                (__tmp_40, __tmp_41) => {
                    (*__tmp_41) = (((*__tmp_41) as i32) & (__tmp_40 as i32)) != 0;
                }
            }
        };
        {
            match (
                {
                    (((*first_it.borrow())
                        == equivalent.as_pointer().with(
                            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                                __m.find(&equivalent_a.as_pointer())
                            },
                        )) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_42, __tmp_43) => {
                    (*__tmp_43) = (((*__tmp_43) as i32) & (__tmp_42 as i32)) != 0;
                }
            }
        };
        {
            match (
                {
                    (((*first_it.borrow())
                        == equivalent.as_pointer().with(
                            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                                __m.find(&equivalent_b.as_pointer())
                            },
                        )) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_44, __tmp_45) => {
                    (*__tmp_45) = (((*__tmp_45) as i32) & (__tmp_44 as i32)) != 0;
                }
            }
        };
        {
            match (
                {
                    (({
                        ((*({ (*first_it.borrow()).iter_deref() }).upgrade().deref())
                            .1
                            .as_pointer())
                    } == { (first_value).clone() }) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_46, __tmp_47) => {
                    (*__tmp_47) = (((*__tmp_47) as i32) & (__tmp_46 as i32)) != 0;
                }
            }
        };
        {
            match (
                {
                    (((({ (*first_it.borrow()).iter_deref() }).with(|__s| (*__s.1.borrow()).value)
                        == 31)
                        && (second.with(|__s| __s.value) == 31)) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_48, __tmp_49) => {
                    (*__tmp_49) = (((*__tmp_49) as i32) & (__tmp_48 as i32)) != 0;
                }
            }
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
            {
                match (
                    {
                        ((({ (*(*entry.upgrade().deref()).0.borrow()).order } == 3)
                            && ({ (*(*entry.upgrade().deref()).1.borrow()).value } == 31))
                            as i32)
                    },
                    &mut (ok),
                ) {
                    (__tmp_50, __tmp_51) => {
                        (*__tmp_51) = (((*__tmp_51) as i32) & (__tmp_50 as i32)) != 0;
                    }
                }
            };
            equivalent_count.prefix_inc();
        }
        {
            match ({ ((equivalent_count == 1_usize) as i32) }, &mut (ok)) {
                (__tmp_52, __tmp_53) => {
                    (*__tmp_53) = (((*__tmp_53) as i32) & (__tmp_52 as i32)) != 0;
                }
            }
        };
    }
    {
        match (
            {
                (!({
                    let _a: Ptr<Key> = equivalent_a.as_pointer();
                    operator_eq_2(_a, equivalent_b.as_pointer())
                }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_54, __tmp_55) => {
                (*__tmp_55) = (((*__tmp_55) as i32) & (__tmp_54 as i32)) != 0;
            }
        }
    };
    let later_key: Value<Key> = Rc::new(RefCell::new(Key {
        order: 4,
        identity: 10,
    }));
    let later: Ptr<Mapped> =
        ({ (*equivalent.borrow_mut()).subscript(later_key.as_pointer(), || <Mapped>::default()) });
    field!(later, value).write(41);
    {
        match (
            {
                ((default_constructions_0.with(|rc| *rc.borrow()) == (before_equivalent + 2))
                    as i32)
            },
            &mut (ok),
        ) {
            (__tmp_56, __tmp_57) => {
                (*__tmp_57) = (((*__tmp_57) as i32) & (__tmp_56 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (((*equivalent.borrow()).len() == 2_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_58, __tmp_59) => {
                (*__tmp_59) = (((*__tmp_59) as i32) & (__tmp_58 as i32)) != 0;
            }
        }
    };
    if reused {
        {
            match (
                {
                    (((*first_it.borrow())
                        == equivalent.as_pointer().with(
                            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                                __m.find(&equivalent_a.as_pointer())
                            },
                        )) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_60, __tmp_61) => {
                    (*__tmp_61) = (((*__tmp_61) as i32) & (__tmp_60 as i32)) != 0;
                }
            }
        };
        {
            match (
                {
                    (({
                        ((*({ (*first_it.borrow()).iter_deref() }).upgrade().deref())
                            .1
                            .as_pointer())
                    } == { (first_value).clone() }) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_62, __tmp_63) => {
                    (*__tmp_63) = (((*__tmp_63) as i32) & (__tmp_62 as i32)) != 0;
                }
            }
        };
        {
            match (
                {
                    ((({ (*first_it.borrow()).iter_deref() }).with(|__s| (*__s.1.borrow()).value)
                        == 31) as i32)
                },
                &mut (ok),
            ) {
                (__tmp_64, __tmp_65) => {
                    (*__tmp_65) = (((*__tmp_65) as i32) & (__tmp_64 as i32)) != 0;
                }
            }
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
    let matched: Ptr<Mapped> =
        ({ (*matching.borrow_mut()).subscript(matching_a.as_pointer(), || <Mapped>::default()) });
    field!(matched, value).write(51);
    let matched_it: Value<BidirectionalIterator<(Value<Key>, Value<Mapped>)>> =
        Rc::new(RefCell::new(matching.as_pointer().with(
            |__m: &Map<Key, (Value<Key>, Value<Mapped>)>| __m.find(&matching_a.as_pointer()),
        )));
    let mut matched_value: Ptr<Mapped> =
        ((*({ (*matched_it.borrow()).iter_deref() }).upgrade().deref())
            .1
            .as_pointer());
    {
        match (
            { ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1)) as i32) },
            &mut (ok),
        ) {
            (__tmp_66, __tmp_67) => {
                (*__tmp_67) = (((*__tmp_67) as i32) & (__tmp_66 as i32)) != 0;
            }
        }
    };
    let matched_again: Ptr<Mapped> =
        ({ (*matching.borrow_mut()).subscript(matching_b.as_pointer(), || <Mapped>::default()) });
    {
        match (
            { ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 1)) as i32) },
            &mut (ok),
        ) {
            (__tmp_68, __tmp_69) => {
                (*__tmp_69) = (((*__tmp_69) as i32) & (__tmp_68 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (((*matching.borrow()).len() == 1_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_70, __tmp_71) => {
                (*__tmp_71) = (((*__tmp_71) as i32) & (__tmp_70 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (({ (matched).clone() } == { (matched_again).clone() }) as i32) },
            &mut (ok),
        ) {
            (__tmp_72, __tmp_73) => {
                (*__tmp_73) = (((*__tmp_73) as i32) & (__tmp_72 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((*matched_it.borrow())
                    == matching
                        .as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&matching_b.as_pointer())
                        })) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_74, __tmp_75) => {
                (*__tmp_75) = (((*__tmp_75) as i32) & (__tmp_74 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (({
                    ((*({ (*matched_it.borrow()).iter_deref() }).upgrade().deref())
                        .1
                        .as_pointer())
                } == { (matched_value).clone() }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_76, __tmp_77) => {
                (*__tmp_77) = (((*__tmp_77) as i32) & (__tmp_76 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                ((({ (*matched_it.borrow()).iter_deref() }).with(|__s| (*__s.1.borrow()).value)
                    == 51) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_78, __tmp_79) => {
                (*__tmp_79) = (((*__tmp_79) as i32) & (__tmp_78 as i32)) != 0;
            }
        }
    };
    let other: Ptr<Mapped> =
        ({ (*matching.borrow_mut()).subscript(matching_c.as_pointer(), || <Mapped>::default()) });
    field!(other, value).write(61);
    {
        match (
            { ((default_constructions_0.with(|rc| *rc.borrow()) == (before_matching + 2)) as i32) },
            &mut (ok),
        ) {
            (__tmp_80, __tmp_81) => {
                (*__tmp_81) = (((*__tmp_81) as i32) & (__tmp_80 as i32)) != 0;
            }
        }
    };
    {
        match (
            { (((*matching.borrow()).len() == 2_usize) as i32) },
            &mut (ok),
        ) {
            (__tmp_82, __tmp_83) => {
                (*__tmp_83) = (((*__tmp_83) as i32) & (__tmp_82 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (((*matched_it.borrow())
                    == matching
                        .as_pointer()
                        .with(|__m: &Map<Key, (Value<Key>, Value<Mapped>)>| {
                            __m.find(&matching_a.as_pointer())
                        })) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_84, __tmp_85) => {
                (*__tmp_85) = (((*__tmp_85) as i32) & (__tmp_84 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                (({
                    ((*({ (*matched_it.borrow()).iter_deref() }).upgrade().deref())
                        .1
                        .as_pointer())
                } == { (matched_value).clone() }) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_86, __tmp_87) => {
                (*__tmp_87) = (((*__tmp_87) as i32) & (__tmp_86 as i32)) != 0;
            }
        }
    };
    {
        match (
            {
                ((({ (*matched_it.borrow()).iter_deref() }).with(|__s| (*__s.1.borrow()).value)
                    == 51) as i32)
            },
            &mut (ok),
        ) {
            (__tmp_88, __tmp_89) => {
                (*__tmp_89) = (((*__tmp_89) as i32) & (__tmp_88 as i32)) != 0;
            }
        }
    };
    return if ok { 0 } else { 1 };
}
pub fn __cpp2rust_init_globals() {
    let _ = default_constructions_0.with(|_| ());
}
