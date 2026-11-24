extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static calls_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static made_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copies_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moves_3: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct Item {
    #[offset(0)]
    pub value: i32,
}
impl Item {
    pub fn new_2() -> Self {
        Self { value: 0 }
    }
    pub fn new_1(mut value: i32) -> Self {
        let __this: Item = Self { value: value };
        (*made_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(other: Ptr<Item>) -> Self {
        let __this: Item = Self {
            value: other.with(|__s| __s.value),
        };
        (*copies_2.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<Item>) -> Self {
        let __this: Item = Self {
            value: other.with(|__s| __s.value),
        };
        (*moves_3.with(Value::clone).borrow_mut()).prefix_inc();
        field!(other, value).write(-1_i32);
        __this
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let __src: Value<Item> = Rc::new(RefCell::new(Item {
            value: self.value.clone(),
        }));
        Item::copy_from(__src.as_pointer())
    }
}
impl Default for Item {
    fn default() -> Self {
        { Item::new_2() }
    }
}
pub fn argument_4(item: Ptr<Item>) -> Ptr<Item> {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (item).clone();
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct RefItem {
    #[offset(0)]
    pub value: i32,
}
impl RefItem {
    pub fn new_1() -> Self {
        Self { value: 0 }
    }
    pub fn new_2(counter: Ptr<i32>, mut amount: i32) -> Self {
        let __this: RefItem = Self {
            value: match ({ amount }, &(counter)) {
                (__tmp_0, __tmp_1) => __tmp_1.with_mut(|__tmp_1| {
                    (*__tmp_1) = (((*__tmp_1) as i32) + (__tmp_0 as i32)) as i32;
                    (*__tmp_1).clone()
                }),
            },
        };
        (*made_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for RefItem {
    fn default() -> Self {
        { RefItem::new_1() }
    }
}
pub fn reference_5(counter: Ptr<i32>) -> Ptr<i32> {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (counter).clone();
}
thread_local!(
    pub static global_map_6: Value<Map<i32, (Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new(Map::new()));
);
thread_local!(
    pub static global_key_7: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static global_counter_8: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static seed_9: Value<i32> = Rc::new(RefCell::new({
        ({
            (*global_map_6.with(Value::clone).borrow_mut())
                .subscript(global_key_7.with(|v| v.as_pointer()), || 0_i32)
        })
        .write(10);
        0
    }));
);
thread_local!(
    pub static global_hit_10: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match ({
            let __tmp_3 = global_counter_8.with(|v| v.as_pointer());
            __tmp_3.with_mut(|__v| {
                __v.prefix_inc();
            });
            __tmp_3
        },)
        {
            (__tmp_2,) => {
                let (__it, __inserted) = Map::emplace(
                    global_map_6.with(|v| v.as_pointer()),
                    &global_key_7.with(|v| v.as_pointer()),
                    || {
                        (
                            Rc::new(RefCell::new(global_key_7.with(|v| v.as_pointer()).read())),
                            Rc::new(RefCell::new((__tmp_2.read()))),
                        )
                    },
                );
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut ok: bool = ((global_counter_8.with(|rc| *rc.borrow()) == 1)
        && (!(*(*global_hit_10.with(Value::clone).borrow()).1.borrow())))
        && ((*(*({ (*(*global_hit_10.with(Value::clone).borrow()).0.borrow()).iter_deref() })
            .upgrade()
            .deref())
        .1
        .borrow())
            == 10);
    let mut rows: i32 = 0;
    {
        let mut subset: i32 = 0;
        'loop_: while (subset < 8) {
            {
                let key: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*key.borrow()) < 3) {
                    {
                        let mut initial: i32 = 0;
                        'loop_: while (initial < 3) {
                            {
                                let mut post: i32 = 0;
                                'loop_: while (post < 2) {
                                    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                                        Rc::new(RefCell::new(Map::new()));
                                    let mut size: i32 = 0;
                                    {
                                        let k: Value<i32> = Rc::new(RefCell::new(0));
                                        'loop_: while ((*k.borrow()) < 3) {
                                            if ((subset & (1 << (*k.borrow()))) != 0) {
                                                let __rhs = (10 + (*k.borrow()));
                                                ({
                                                    (*m.borrow_mut())
                                                        .subscript(k.as_pointer(), || 0_i32)
                                                })
                                                .write(__rhs);
                                                size.prefix_inc();
                                            }
                                            (*k.borrow_mut()).prefix_inc();
                                        }
                                    }
                                    let counter: Value<i32> = Rc::new(RefCell::new(initial));
                                    let result: Value<(
                                        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
                                        Value<bool>,
                                    )> = Rc::new(RefCell::new({
                                        let __tmp_5: Value<i32>;
                                        if (post != 0) {
                                            match ({
                                                __tmp_5 = Rc::new(RefCell::new(
                                                    (*counter.borrow_mut()).postfix_inc(),
                                                ));
                                                __tmp_5.as_pointer()
                                            },)
                                            {
                                                (__tmp_4,) => {
                                                    let (__it, __inserted) = Map::emplace(
                                                        m.as_pointer(),
                                                        &key.as_pointer(),
                                                        || {
                                                            (
                                                                Rc::new(RefCell::new(
                                                                    key.as_pointer().read(),
                                                                )),
                                                                Rc::new(RefCell::new(
                                                                    (__tmp_4.read()),
                                                                )),
                                                            )
                                                        },
                                                    );
                                                    (
                                                        Rc::new(RefCell::new(__it)),
                                                        Rc::new(RefCell::new(__inserted)),
                                                    )
                                                }
                                            }
                                        } else {
                                            match ({
                                                let __tmp_7 = counter.as_pointer();
                                                __tmp_7.with_mut(|__v| {
                                                    __v.prefix_inc();
                                                });
                                                __tmp_7
                                            },)
                                            {
                                                (__tmp_6,) => {
                                                    let (__it, __inserted) = Map::emplace(
                                                        m.as_pointer(),
                                                        &key.as_pointer(),
                                                        || {
                                                            (
                                                                Rc::new(RefCell::new(
                                                                    key.as_pointer().read(),
                                                                )),
                                                                Rc::new(RefCell::new(
                                                                    (__tmp_6.read()),
                                                                )),
                                                            )
                                                        },
                                                    );
                                                    (
                                                        Rc::new(RefCell::new(__it)),
                                                        Rc::new(RefCell::new(__inserted)),
                                                    )
                                                }
                                            }
                                        }
                                    }));
                                    let mut hit: bool = ((subset & (1 << (*key.borrow()))) != 0);
                                    match (
                                        { (((*counter.borrow()) == (initial + 1)) as i32) },
                                        &mut (ok),
                                    ) {
                                        (__tmp_8, __tmp_9) => {
                                            (*__tmp_9) =
                                                (((*__tmp_9) as i32) & (__tmp_8 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        {
                                            ((((*(*result.borrow()).1.borrow()) as i32)
                                                == (!(hit) as i32))
                                                as i32)
                                        },
                                        &mut (ok),
                                    ) {
                                        (__tmp_10, __tmp_11) => {
                                            (*__tmp_11) =
                                                (((*__tmp_11) as i32) & (__tmp_10 as i32)) != 0;
                                        }
                                    };
                                    let mut found: bool = (*(*result.borrow()).0.borrow())
                                        == m.as_pointer().with(
                                            |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                                                __m.find(&key.as_pointer())
                                            },
                                        );
                                    match ({ (found as i32) }, &mut (ok)) {
                                        (__tmp_12, __tmp_13) => {
                                            (*__tmp_13) =
                                                (((*__tmp_13) as i32) & (__tmp_12 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        {
                                            (((*(*({
                                                (*(*result.borrow()).0.borrow()).iter_deref()
                                            })
                                            .upgrade()
                                            .deref())
                                            .0
                                            .borrow())
                                                == (*key.borrow()))
                                                as i32)
                                        },
                                        &mut (ok),
                                    ) {
                                        (__tmp_14, __tmp_15) => {
                                            (*__tmp_15) =
                                                (((*__tmp_15) as i32) & (__tmp_14 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        {
                                            (((*(*({
                                                (*(*result.borrow()).0.borrow()).iter_deref()
                                            })
                                            .upgrade()
                                            .deref())
                                            .1
                                            .borrow())
                                                == (if hit {
                                                    (10 + (*key.borrow()))
                                                } else {
                                                    (initial + (!(post != 0) as i32))
                                                }))
                                                as i32)
                                        },
                                        &mut (ok),
                                    ) {
                                        (__tmp_16, __tmp_17) => {
                                            (*__tmp_17) =
                                                (((*__tmp_17) as i32) & (__tmp_16 as i32)) != 0;
                                        }
                                    };
                                    match (
                                        {
                                            (((*m.borrow()).len()
                                                == ((size + (!(hit) as i32)) as usize))
                                                as i32)
                                        },
                                        &mut (ok),
                                    ) {
                                        (__tmp_18, __tmp_19) => {
                                            (*__tmp_19) =
                                                (((*__tmp_19) as i32) & (__tmp_18 as i32)) != 0;
                                        }
                                    };
                                    rows.prefix_inc();
                                    post.prefix_inc();
                                }
                            }
                            initial.prefix_inc();
                        }
                    }
                    (*key.borrow_mut()).prefix_inc();
                }
            }
            subset.prefix_inc();
        }
    }
    let items: Value<Map<i32, (Value<i32>, Value<Item>)>> = Rc::new(RefCell::new(Map::new()));
    let key: Value<i32> = Rc::new(RefCell::new(0));
    {
        let (__it, __inserted) = Map::emplace(items.as_pointer(), &key.as_pointer(), || {
            (
                Rc::new(RefCell::new(key.as_pointer().read())),
                Rc::new(RefCell::new(Item::new_1({ 10 }))),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    let source: Value<Item> = Rc::new(RefCell::new(Item::new_1({ 20 })));
    made_1.with(|rc| {
        *rc.borrow_mut() = match (
            {
                match ({ 0 }, &mut (*moves_3.with(Value::clone).borrow_mut())) {
                    (__tmp_22, __tmp_23) => {
                        (*__tmp_23) = __tmp_22;
                        (*__tmp_23).clone()
                    }
                }
            },
            &mut (*copies_2.with(Value::clone).borrow_mut()),
        ) {
            (__tmp_20, __tmp_21) => {
                (*__tmp_21) = __tmp_20;
                (*__tmp_21).clone()
            }
        }
    });
    let hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_24,) => {
                let (__it, __inserted) =
                    Map::emplace(items.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(Item::copy_from({ (__tmp_24).clone() }))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    match (
        {
            (((((calls_0.with(|rc| *rc.borrow()) == 1) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 0)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_25, __tmp_26) => {
            (*__tmp_26) = (((*__tmp_26) as i32) & (__tmp_25 as i32)) != 0;
        }
    };
    match (
        {
            (((!(*(*hit.borrow()).1.borrow()))
                && (({ (*(*hit.borrow()).0.borrow()).iter_deref() })
                    .with(|__s| (*__s.1.borrow()).value)
                    == 10)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_27, __tmp_28) => {
            (*__tmp_28) = (((*__tmp_28) as i32) & (__tmp_27 as i32)) != 0;
        }
    };
    let move_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_29,) => {
                let (__it, __inserted) =
                    Map::emplace(items.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(Item::move_from({ (__tmp_29).clone() }))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    match (
        {
            (((((calls_0.with(|rc| *rc.borrow()) == 2) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 0)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_30, __tmp_31) => {
            (*__tmp_31) = (((*__tmp_31) as i32) & (__tmp_30 as i32)) != 0;
        }
    };
    match (
        {
            (((!(*(*move_hit.borrow()).1.borrow())) && ({ (*source.borrow()).value } == 20)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_32, __tmp_33) => {
            (*__tmp_33) = (((*__tmp_33) as i32) & (__tmp_32 as i32)) != 0;
        }
    };
    (*key.borrow_mut()).prefix_inc();
    let copied: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_34,) => {
                let (__it, __inserted) =
                    Map::emplace(items.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(Item::copy_from({ (__tmp_34).clone() }))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    match (
        {
            (((((calls_0.with(|rc| *rc.borrow()) == 3) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 1))
                && (moves_3.with(|rc| *rc.borrow()) == 0)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_35, __tmp_36) => {
            (*__tmp_36) = (((*__tmp_36) as i32) & (__tmp_35 as i32)) != 0;
        }
    };
    match (
        {
            (((*(*copied.borrow()).1.borrow())
                && (({ (*(*copied.borrow()).0.borrow()).iter_deref() })
                    .with(|__s| (*__s.1.borrow()).value)
                    == 20)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_37, __tmp_38) => {
            (*__tmp_38) = (((*__tmp_38) as i32) & (__tmp_37 as i32)) != 0;
        }
    };
    (*key.borrow_mut()).prefix_inc();
    let moved: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_39,) => {
                let (__it, __inserted) =
                    Map::emplace(items.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(Item::move_from({ (__tmp_39).clone() }))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    match (
        {
            (((((calls_0.with(|rc| *rc.borrow()) == 4) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 1))
                && (moves_3.with(|rc| *rc.borrow()) == 1)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_40, __tmp_41) => {
            (*__tmp_41) = (((*__tmp_41) as i32) & (__tmp_40 as i32)) != 0;
        }
    };
    match (
        {
            ((((*(*moved.borrow()).1.borrow())
                && (({ (*(*moved.borrow()).0.borrow()).iter_deref() })
                    .with(|__s| (*__s.1.borrow()).value)
                    == 20))
                && ({ (*source.borrow()).value } == -1_i32)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_42, __tmp_43) => {
            (*__tmp_43) = (((*__tmp_43) as i32) & (__tmp_42 as i32)) != 0;
        }
    };
    let refs: Value<Map<i32, (Value<i32>, Value<RefItem>)>> = Rc::new(RefCell::new(Map::new()));
    (*key.borrow_mut()) = 0;
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    {
        let (__it, __inserted) = Map::emplace(refs.as_pointer(), &key.as_pointer(), || {
            (
                Rc::new(RefCell::new(key.as_pointer().read())),
                Rc::new(RefCell::new(RefItem::new_2({ counter.as_pointer() }, {
                    5
                }))),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    made_1.with(|rc| *rc.borrow_mut() = 0);
    let ref_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<RefItem>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ reference_5(counter.as_pointer()) }),) {
            (__tmp_44,) => {
                let (__it, __inserted) = Map::emplace(refs.as_pointer(), &key.as_pointer(), || {
                    (
                        Rc::new(RefCell::new(key.as_pointer().read())),
                        Rc::new(RefCell::new(RefItem::new_2({ (__tmp_44).clone() }, { 2 }))),
                    )
                });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    match (
        {
            (((((calls_0.with(|rc| *rc.borrow()) == 5) && ((*counter.borrow()) == 5))
                && (made_1.with(|rc| *rc.borrow()) == 0))
                && (!(*(*ref_hit.borrow()).1.borrow()))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_45, __tmp_46) => {
            (*__tmp_46) = (((*__tmp_46) as i32) & (__tmp_45 as i32)) != 0;
        }
    };
    (*key.borrow_mut()).prefix_inc();
    let ref_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<RefItem>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ reference_5(counter.as_pointer()) }),) {
            (__tmp_47,) => {
                let (__it, __inserted) = Map::emplace(refs.as_pointer(), &key.as_pointer(), || {
                    (
                        Rc::new(RefCell::new(key.as_pointer().read())),
                        Rc::new(RefCell::new(RefItem::new_2({ (__tmp_47).clone() }, { 2 }))),
                    )
                });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    match (
        {
            (((((calls_0.with(|rc| *rc.borrow()) == 6) && ((*counter.borrow()) == 7))
                && (made_1.with(|rc| *rc.borrow()) == 1))
                && (*(*ref_new.borrow()).1.borrow())) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_48, __tmp_49) => {
            (*__tmp_49) = (((*__tmp_49) as i32) & (__tmp_48 as i32)) != 0;
        }
    };
    match (
        {
            (((({ (*(*ref_hit.borrow()).0.borrow()).iter_deref() })
                .with(|__s| (*__s.1.borrow()).value)
                == 5)
                && (({ (*(*ref_new.borrow()).0.borrow()).iter_deref() })
                    .with(|__s| (*__s.1.borrow()).value)
                    == 7)) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_50, __tmp_51) => {
            (*__tmp_51) = (((*__tmp_51) as i32) & (__tmp_50 as i32)) != 0;
        }
    };
    (*key.borrow_mut()) = 0;
    made_1.with(|rc| {
        *rc.borrow_mut() = match (
            {
                match ({ 0 }, &mut (*moves_3.with(Value::clone).borrow_mut())) {
                    (__tmp_54, __tmp_55) => {
                        (*__tmp_55) = __tmp_54;
                        (*__tmp_55).clone()
                    }
                }
            },
            &mut (*copies_2.with(Value::clone).borrow_mut()),
        ) {
            (__tmp_52, __tmp_53) => {
                (*__tmp_53) = __tmp_52;
                (*__tmp_53).clone()
            }
        }
    });
    let temporary_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __tmp_57: Value<Item>;
        match ({
            __tmp_57 = Rc::new(RefCell::new(Item::new_1({ 30 })));
            __tmp_57.as_pointer()
        },)
        {
            (__tmp_56,) => {
                let (__it, __inserted) =
                    Map::emplace(items.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(Item::move_from({ (__tmp_56).clone() }))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        }
    }));
    match (
        {
            (((((made_1.with(|rc| *rc.borrow()) == 1) && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 0))
                && (!(*(*temporary_hit.borrow()).1.borrow()))) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_58, __tmp_59) => {
            (*__tmp_59) = (((*__tmp_59) as i32) & (__tmp_58 as i32)) != 0;
        }
    };
    (*key.borrow_mut()).prefix_inc();
    (*key.borrow_mut()).prefix_inc();
    (*key.borrow_mut()).prefix_inc();
    let temporary_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __tmp_61: Value<Item>;
        match ({
            __tmp_61 = Rc::new(RefCell::new(Item::new_1({ 40 })));
            __tmp_61.as_pointer()
        },)
        {
            (__tmp_60,) => {
                let (__it, __inserted) =
                    Map::emplace(items.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(Item::move_from({ (__tmp_60).clone() }))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        }
    }));
    match (
        {
            (((((made_1.with(|rc| *rc.borrow()) == 2) && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 1))
                && (*(*temporary_new.borrow()).1.borrow())) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_62, __tmp_63) => {
            (*__tmp_63) = (((*__tmp_63) as i32) & (__tmp_62 as i32)) != 0;
        }
    };
    match (
        {
            ((({ (*(*temporary_new.borrow()).0.borrow()).iter_deref() })
                .with(|__s| (*__s.1.borrow()).value)
                == 40) as i32)
        },
        &mut (ok),
    ) {
        (__tmp_64, __tmp_65) => {
            (*__tmp_65) = (((*__tmp_65) as i32) & (__tmp_64 as i32)) != 0;
        }
    };
    printf!(
        b"%d %d %d %d %d\n",
        rows,
        calls_0.with(|rc| *rc.borrow()),
        made_1.with(|rc| *rc.borrow()),
        copies_2.with(|rc| *rc.borrow()),
        moves_3.with(|rc| *rc.borrow())
    );
    return if ok { 0 } else { 1 };
}
pub fn __cpp2rust_init_globals() {
    let _ = calls_0.with(|_| ());
    let _ = made_1.with(|_| ());
    let _ = copies_2.with(|_| ());
    let _ = moves_3.with(|_| ());
    let _ = global_map_6.with(|_| ());
    let _ = global_key_7.with(|_| ());
    let _ = global_counter_8.with(|_| ());
    let _ = seed_9.with(|_| ());
    let _ = global_hit_10.with(|_| ());
}
