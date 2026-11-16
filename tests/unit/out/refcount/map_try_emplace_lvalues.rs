extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static made_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static visits_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Item {
    #[offset(0)]
    pub value: i32,
}
impl Item {
    pub fn new_1() -> Self {
        Self { value: 0 }
    }
    pub fn new_2(counter: Ptr<i32>, mut amount: i32) -> Self {
        let __this: Item = Self {
            value: {
                {
                    let __rhs = { amount };
                    counter.with_mut(|__v| *__v = *__v + __rhs)
                };
                (counter.read())
            },
        };
        (*made_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for Item {
    fn default() -> Self {
        { Item::new_1() }
    }
}
pub fn receiver_2(
    m: Ptr<Map<i32, (Value<i32>, Value<Item>)>>,
) -> Ptr<Map<i32, (Value<i32>, Value<Item>)>> {
    (*visits_1.with(Value::clone).borrow_mut()).prefix_inc();
    return (m).clone();
}
pub fn place_3(n: Ptr<i32>) -> Ptr<i32> {
    (*visits_1.with(Value::clone).borrow_mut()).prefix_inc();
    return (n).clone();
}
thread_local!(
    pub static temporary_calls_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Temporary {
    #[offset(0)]
    pub n: i32,
    #[offset(4)]
    #[byte_size(8)]
    pub data: Box<[i32]>,
}
impl Temporary {
    pub fn new() -> Self {
        let __this: Temporary = Self {
            n: 5,
            data: Box::new([6, 7]),
        };
        (*temporary_calls_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for Temporary {
    fn default() -> Self {
        { Temporary::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Nested {
    #[offset(0)]
    #[byte_size(12)]
    pub inner: Temporary,
}
pub fn make_temporary_5() -> Temporary {
    return Temporary::new();
}
pub fn make_nested_6() -> Nested {
    return <Nested>::default();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Option<Value<Temporary>>,
}
impl Owner {
    pub fn new() -> Self {
        Self {
            ptr: Some(Rc::new(RefCell::new(Temporary::new()))),
        }
    }
    pub fn move_from(_a0: Ptr<Owner>) -> Self {
        Self {
            ptr: field!(_a0, ptr).with_mut(|__v: &mut Option<Value<Temporary>>| __v.take()),
        }
    }
}
impl Default for Owner {
    fn default() -> Self {
        { Owner::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct Noncopy {
    #[offset(0)]
    pub n: i32,
}
impl Default for Noncopy {
    fn default() -> Self {
        { Noncopy { n: 5 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, ptr)]
#[byte_size(8)]
pub struct Held {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<i32>,
}
impl Held {
    pub fn new_2() -> Self {
        Self {
            ptr: Ptr::<i32>::null(),
        }
    }
    pub fn new_1(n: Ptr<i32>) -> Self {
        Self { ptr: (n).clone() }
    }
    pub fn new_3(n: Ptr<i32>, mut _a1: i32) -> Self {
        Self { ptr: (n).clone() }
    }
}
impl Default for Held {
    fn default() -> Self {
        { Held::new_2() }
    }
}
thread_local!(
    pub static global_values_7: Value<Map<i32, (Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new(Map::new()));
);
thread_local!(
    pub static global_key_8: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static conditional_key_9: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static owned_key_10: Value<i32> = Rc::new(RefCell::new(2));
);
thread_local!(
    pub static global_a_11: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static global_b_12: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static global_choose_13: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static global_temporary_14: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __tmp_2: Value<Temporary>;
        match ((match {
            __tmp_2 = Rc::new(RefCell::new(({ make_temporary_5() })));
            __tmp_2.as_pointer()
        } {
            __tmp_1 => field_ptr!(__tmp_1, n),
        }),)
        {
            (__tmp_0,) => {
                let (__it, __inserted) = Map::emplace(
                    global_values_7.with(|v| v.as_pointer()),
                    &global_key_8.with(|v| v.as_pointer()),
                    || {
                        (
                            Rc::new(RefCell::new(global_key_8.with(|v| v.as_pointer()).read())),
                            Rc::new(RefCell::new((__tmp_0.read()))),
                        )
                    },
                );
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        }
    }));
);
thread_local!(
    pub static global_conditional_15: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (if global_choose_13.with(|rc| *rc.borrow()) {
            {
                let __tmp_4 = global_a_11.with(|v| v.as_pointer());
                __tmp_4.with_mut(|__v| __v.prefix_inc());
                __tmp_4
            }
        } else {
            {
                let __tmp_5 = global_b_12.with(|v| v.as_pointer());
                __tmp_5.with_mut(|__v| __v.prefix_inc());
                __tmp_5
            }
        },)
        {
            (__tmp_3,) => {
                let (__it, __inserted) = Map::emplace(
                    global_values_7.with(|v| v.as_pointer()),
                    &conditional_key_9.with(|v| v.as_pointer()),
                    || {
                        (
                            Rc::new(RefCell::new(
                                conditional_key_9.with(|v| v.as_pointer()).read(),
                            )),
                            Rc::new(RefCell::new((__tmp_3.read()))),
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
thread_local!(
    pub static global_owned_16: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let mut __tmp_7: Option<Value<Temporary>>;
        match (field_ptr!(
            (*{
                __tmp_7 = Some(Rc::new(RefCell::new(Temporary::new())));
                &mut __tmp_7
            })
            .as_pointer(),
            n
        ),)
        {
            (__tmp_6,) => {
                let (__it, __inserted) = Map::emplace(
                    global_values_7.with(|v| v.as_pointer()),
                    &owned_key_10.with(|v| v.as_pointer()),
                    || {
                        (
                            Rc::new(RefCell::new(owned_key_10.with(|v| v.as_pointer()).read())),
                            Rc::new(RefCell::new((__tmp_6.read()))),
                        )
                    },
                );
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        }
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        ({
            (*(*global_temporary_14.with(Value::clone).borrow())
                .1
                .borrow())
                && ((*(*(*(*global_temporary_14.with(Value::clone).borrow())
                    .0
                    .borrow())
                .iter_deref()
                .upgrade()
                .deref())
                .1
                .borrow())
                    == 5)
        })
    );
    assert!(
        ({
            (*(*global_owned_16.with(Value::clone).borrow()).1.borrow())
                && ((*(*(*(*global_owned_16.with(Value::clone).borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 5)
        })
    );
    assert!(
        ({
            (((*(*global_conditional_15.with(Value::clone).borrow())
                .1
                .borrow())
                && ((*(*(*(*global_conditional_15.with(Value::clone).borrow())
                    .0
                    .borrow())
                .iter_deref()
                .upgrade()
                .deref())
                .1
                .borrow())
                    == 1))
                && (global_a_11.with(|rc| *rc.borrow()) == 0))
                && (global_b_12.with(|rc| *rc.borrow()) == 1)
        })
    );
    let m: Value<Map<i32, (Value<i32>, Value<Item>)>> = Rc::new(RefCell::new(Map::new()));
    let key: Value<i32> = Rc::new(RefCell::new(0));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    {
        let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
            (
                Rc::new(RefCell::new(key.as_pointer().read())),
                Rc::new(RefCell::new(Item::new_2({ counter.as_pointer() }, { 3 }))),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    made_0.with(|rc| *rc.borrow_mut() = 0);
    let hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ receiver_2(m.as_pointer()) }), {
            let __tmp_10 = ({ place_3(counter.as_pointer()) });
            __tmp_10.with_mut(|__v| __v.prefix_inc());
            __tmp_10
        }) {
            (__tmp_8, __tmp_9) => {
                let (__it, __inserted) = Map::emplace((__tmp_8).clone(), &key.as_pointer(), || {
                    (
                        Rc::new(RefCell::new(key.as_pointer().read())),
                        Rc::new(RefCell::new(Item::new_2({ (__tmp_9).clone() }, { 2 }))),
                    )
                });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ((((*counter.borrow()) == 4) && (made_0.with(|rc| *rc.borrow()) == 0))
                && (visits_1.with(|rc| *rc.borrow()) == 2))
                && (!(*(*hit.borrow()).1.borrow()))
        })
    );
    (*key.borrow_mut()).prefix_inc();
    let inserted: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match ({
            let __tmp_12 = counter.as_pointer();
            __tmp_12.with_mut(|__v| __v.prefix_inc());
            __tmp_12
        },)
        {
            (__tmp_11,) => {
                let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                    (
                        Rc::new(RefCell::new(key.as_pointer().read())),
                        Rc::new(RefCell::new(Item::new_2({ (__tmp_11).clone() }, { 2 }))),
                    )
                });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            (((*counter.borrow()) == 7) && (made_0.with(|rc| *rc.borrow()) == 1))
                && (*(*inserted.borrow()).1.borrow())
        })
    );
    let assigned: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (match ({ 9 }, counter.as_pointer()) {
            (__tmp_15, __tmp_14) => {
                __tmp_14.write(__tmp_15);
                __tmp_14
            }
        },)
        {
            (__tmp_13,) => {
                let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                    (
                        Rc::new(RefCell::new(key.as_pointer().read())),
                        Rc::new(RefCell::new(Item::new_2({ (__tmp_13).clone() }, { 2 }))),
                    )
                });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            (((*counter.borrow()) == 9) && (made_0.with(|rc| *rc.borrow()) == 1))
                && (!(*(*assigned.borrow()).1.borrow()))
        })
    );
    (*key.borrow_mut()).prefix_inc();
    let compound: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (match ({ 4 }, counter.as_pointer()) {
            (__tmp_18, __tmp_17) => {
                __tmp_17.with_mut(|__v| *__v = (((*__v) as i32) + (__tmp_18 as i32)) as i32);
                __tmp_17
            }
        },)
        {
            (__tmp_16,) => {
                let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                    (
                        Rc::new(RefCell::new(key.as_pointer().read())),
                        Rc::new(RefCell::new(Item::new_2({ (__tmp_16).clone() }, { 2 }))),
                    )
                });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            (((*counter.borrow()) == 15) && (made_0.with(|rc| *rc.borrow()) == 2))
                && (*(*compound.borrow()).1.borrow())
        })
    );
    assert!(
        ({
            ((*(*inserted.borrow()).0.borrow())
                .iter_deref()
                .with(|__s| (*({ __s }).1.borrow()).value)
                == 7)
                && ((*(*compound.borrow()).0.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 15)
        })
    );
    let array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let mut index: i32 = 0;
    (*key.borrow_mut()).prefix_inc();
    match ({
        let __tmp_20 = array.decay().offset(index.postfix_inc());
        __tmp_20.with_mut(|__v| __v.prefix_inc());
        __tmp_20
    },)
    {
        (__tmp_19,) => {
            let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                (
                    Rc::new(RefCell::new(key.as_pointer().read())),
                    Rc::new(RefCell::new(Item::new_2({ (__tmp_19).clone() }, { 3 }))),
                )
            });
            (
                Rc::new(RefCell::new(__it)),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(
        ({
            ((index == 1) && ((*array.borrow())[(0) as usize] == 4))
                && (made_0.with(|rc| *rc.borrow()) == 3)
        })
    );
    (*key.borrow_mut()).prefix_inc();
    match (match ({ 5 }, array.decay().offset(index.postfix_inc())) {
        (__tmp_23, __tmp_22) => {
            __tmp_22.write(__tmp_23);
            __tmp_22
        }
    },)
    {
        (__tmp_21,) => {
            let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                (
                    Rc::new(RefCell::new(key.as_pointer().read())),
                    Rc::new(RefCell::new(Item::new_2({ (__tmp_21).clone() }, { 3 }))),
                )
            });
            (
                Rc::new(RefCell::new(__it)),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(
        ({
            ((index == 2) && ((*array.borrow())[(1) as usize] == 8))
                && (made_0.with(|rc| *rc.borrow()) == 4)
        })
    );
    match (
        match ({ ((*counter.borrow()) + 1) }, counter.as_pointer()) {
            (__tmp_26, __tmp_25) => {
                __tmp_25.write(__tmp_26);
                __tmp_25
            }
        },
    ) {
        (__tmp_24,) => {
            let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                (
                    Rc::new(RefCell::new(key.as_pointer().read())),
                    Rc::new(RefCell::new(Item::new_2({ (__tmp_24).clone() }, { 3 }))),
                )
            });
            (
                Rc::new(RefCell::new(__it)),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(({ ((*counter.borrow()) == 16) && (made_0.with(|rc| *rc.borrow()) == 4) }));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    let b: Value<i32> = Rc::new(RefCell::new(0));
    let mut choose: bool = false;
    match (if choose {
        {
            let __tmp_28 = a.as_pointer();
            __tmp_28.with_mut(|__v| __v.prefix_inc());
            __tmp_28
        }
    } else {
        {
            let __tmp_29 = b.as_pointer();
            __tmp_29.with_mut(|__v| __v.prefix_inc());
            __tmp_29
        }
    },)
    {
        (__tmp_27,) => {
            let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                (
                    Rc::new(RefCell::new(key.as_pointer().read())),
                    Rc::new(RefCell::new(Item::new_2({ (__tmp_27).clone() }, { 3 }))),
                )
            });
            (
                Rc::new(RefCell::new(__it)),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(
        ({
            (((*a.borrow()) == 0) && ((*b.borrow()) == 1)) && (made_0.with(|rc| *rc.borrow()) == 4)
        })
    );
    choose = true;
    match (if choose {
        {
            let __tmp_31 = a.as_pointer();
            __tmp_31.with_mut(|__v| __v.prefix_inc());
            __tmp_31
        }
    } else {
        {
            let __tmp_32 = b.as_pointer();
            __tmp_32.with_mut(|__v| __v.prefix_inc());
            __tmp_32
        }
    },)
    {
        (__tmp_30,) => {
            let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                (
                    Rc::new(RefCell::new(key.as_pointer().read())),
                    Rc::new(RefCell::new(Item::new_2({ (__tmp_30).clone() }, { 3 }))),
                )
            });
            (
                Rc::new(RefCell::new(__it)),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(
        ({
            (((*a.borrow()) == 1) && ((*b.borrow()) == 1)) && (made_0.with(|rc| *rc.borrow()) == 4)
        })
    );
    match ({
        (*counter.borrow_mut()).postfix_inc();
        counter.as_pointer()
    },)
    {
        (__tmp_33,) => {
            let (__it, __inserted) = Map::emplace(m.as_pointer(), &key.as_pointer(), || {
                (
                    Rc::new(RefCell::new(key.as_pointer().read())),
                    Rc::new(RefCell::new(Item::new_2({ (__tmp_33).clone() }, { 3 }))),
                )
            });
            (
                Rc::new(RefCell::new(__it)),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(({ ((*counter.borrow()) == 17) && (made_0.with(|rc| *rc.borrow()) == 4) }));
    let unsigned_map: Value<Map<i32, (Value<i32>, Value<u32>)>> = Rc::new(RefCell::new(Map::new()));
    let n: Value<u32> = Rc::new(RefCell::new(!0_u32));
    let wrapped: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<u32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (match ({ 1_u32 }, n.as_pointer()) {
            (__tmp_36, __tmp_35) => {
                __tmp_35.with_mut(|__v| {
                    *__v = (((*__v) as u32).wrapping_add((__tmp_36 as u32))) as u32
                });
                __tmp_35
            }
        },)
        {
            (__tmp_34,) => {
                let (__it, __inserted) =
                    Map::emplace(unsigned_map.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new((__tmp_34.read()))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ((*n.borrow()) == 0_u32)
                && ((*(*(*(*wrapped.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 0_u32)
        })
    );
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([255_u8, 0_u8])));
    index = 0;
    (*key.borrow_mut()).prefix_inc();
    let narrowed: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<u32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (match ({ 2 }, bytes.decay().offset(index.postfix_inc())) {
            (__tmp_39, __tmp_38) => {
                __tmp_38.with_mut(|__v| *__v = (((*__v) as i32) + (__tmp_39 as i32)) as u8);
                __tmp_38
            }
        },)
        {
            (__tmp_37,) => {
                let (__it, __inserted) =
                    Map::emplace(unsigned_map.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(((__tmp_37.read()) as u32))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ((index == 1) && (((*bytes.borrow())[(0) as usize] as i32) == 1))
                && ((*(*(*(*narrowed.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 1_u32)
        })
    );
    let changed: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript(changed.as_pointer(), key.as_pointer()).write(10);
    let effects: Value<i32> = Rc::new(RefCell::new(0));
    let erased: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match ({
            Map::erase_key(changed.as_pointer(), &key.as_pointer());
            {
                let __tmp_41 = effects.as_pointer();
                __tmp_41.with_mut(|__v| __v.prefix_inc());
                __tmp_41
            }
        },)
        {
            (__tmp_40,) => {
                let (__it, __inserted) =
                    Map::emplace(changed.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new((__tmp_40.read()))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            (((*effects.borrow()) == 1) && (*(*erased.borrow()).1.borrow()))
                && ((*(*(*(*erased.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 1)
        })
    );
    let updated: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match ({
            Map::subscript(changed.as_pointer(), key.as_pointer()).with_mut(|__v| __v.prefix_inc());
            {
                let __tmp_43 = effects.as_pointer();
                __tmp_43.with_mut(|__v| __v.prefix_inc());
                __tmp_43
            }
        },)
        {
            (__tmp_42,) => {
                let (__it, __inserted) =
                    Map::emplace(changed.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new((__tmp_42.read()))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            (((*effects.borrow()) == 2) && (!(*(*updated.borrow()).1.borrow())))
                && ((*(*(*(*updated.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == 2)
        })
    );
    let pointers: Value<Map<i32, (Value<i32>, Value<Ptr<i32>>)>> =
        Rc::new(RefCell::new(Map::new()));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(array.decay()));
    let pointer: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Ptr<i32>>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (match ({ 1 }, p.as_pointer()) {
            (__tmp_46, __tmp_45) => {
                __tmp_45.with_mut(|__v| *__v = (*__v).offset((__tmp_46 as isize)));
                __tmp_45
            }
        },)
        {
            (__tmp_44,) => {
                let (__it, __inserted) =
                    Map::emplace(pointers.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new((__tmp_44.read()))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ({ (*p.borrow()).clone() } == { array.decay().offset((1) as isize) })
                && ({
                    (*(*(*(*pointer.borrow()).0.borrow())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .borrow())
                    .clone()
                } == { (*p.borrow()).clone() })
        })
    );
    let mut hit: i32 = 0;
    'loop_: while (hit < 2) {
        let values: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        let keys: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5])));
        if (hit != 0) {
            'loop_: for k in IterRange::new(keys.decay(), keys.decay().offset((6_i64) as isize)) {
                let k: Value<i32> = Rc::new(RefCell::new((k.read())));
                Map::subscript(values.as_pointer(), k.as_pointer()).write(42);
            }
        }
        let moved: Value<i32> = Rc::new(RefCell::new(0));
        let forwarded: Value<i32> = Rc::new(RefCell::new(0));
        let xvalue: Value<i32> = Rc::new(RefCell::new(0));
        let lvalue: Value<i32> = Rc::new(RefCell::new(0));
        let assigned: Value<i32> = Rc::new(RefCell::new(0));
        let nested: Value<i32> = Rc::new(RefCell::new(0));
        let r0: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            match ({
                let __tmp_48 = moved.as_pointer();
                __tmp_48.with_mut(|__v| __v.prefix_inc());
                __tmp_48
            },)
            {
                (__tmp_47,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(0), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(0).read())),
                                Rc::new(RefCell::new((__tmp_47.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            },
        ));
        let r1: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            match ({
                let __tmp_50 = forwarded.as_pointer();
                __tmp_50.with_mut(|__v| __v.prefix_inc());
                __tmp_50
            },)
            {
                (__tmp_49,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(1), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(1).read())),
                                Rc::new(RefCell::new((__tmp_49.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            },
        ));
        let r2: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            match ({
                let __tmp_52 = xvalue.as_pointer();
                __tmp_52.with_mut(|__v| __v.prefix_inc());
                __tmp_52
            },)
            {
                (__tmp_51,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(2), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(2).read())),
                                Rc::new(RefCell::new((__tmp_51.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            },
        ));
        let r3: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            match ({
                let __tmp_54 = lvalue.as_pointer();
                __tmp_54.with_mut(|__v| __v.prefix_inc());
                __tmp_54
            },)
            {
                (__tmp_53,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(3), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(3).read())),
                                Rc::new(RefCell::new((__tmp_53.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            },
        ));
        let r4: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            match (match ({ 5 }, assigned.as_pointer()) {
                (__tmp_57, __tmp_56) => {
                    __tmp_56.write(__tmp_57);
                    __tmp_56
                }
            },)
            {
                (__tmp_55,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(4), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(4).read())),
                                Rc::new(RefCell::new((__tmp_55.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            },
        ));
        let r5: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(
            match (match ({ 1 }, nested.as_pointer()) {
                (__tmp_60, __tmp_59) => {
                    __tmp_59.with_mut(|__v| *__v = (((*__v) as i32) + (__tmp_60 as i32)) as i32);
                    __tmp_59
                }
            },)
            {
                (__tmp_58,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(5), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(5).read())),
                                Rc::new(RefCell::new((__tmp_58.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            },
        ));
        assert!(
            ({
                ((((((*moved.borrow()) == 1) && ((*forwarded.borrow()) == 1))
                    && ((*xvalue.borrow()) == 1))
                    && ((*lvalue.borrow()) == 1))
                    && ((*assigned.borrow()) == 5))
                    && ((*nested.borrow()) == 1)
            })
        );
        assert!(
            ({
                (((((((*(*r0.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
                    && (((*(*r1.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*r2.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*r3.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*r4.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*r5.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
            })
        );
        'loop_: for k in IterRange::new(keys.decay(), keys.decay().offset((6_i64) as isize)) {
            let k: Value<i32> = Rc::new(RefCell::new((k.read())));
            assert!(
                ({
                    ((Map::subscript(values.as_pointer(), k.as_pointer()).read())
                        == (if (hit != 0) {
                            42
                        } else {
                            if ((*k.borrow()) == 4) { 5 } else { 1 }
                        }))
                })
            );
        }
        hit.prefix_inc();
    }
    let mut hit: i32 = 0;
    'loop_: while (hit < 2) {
        let mut choose: i32 = 0;
        'loop_: while (choose < 2) {
            let selected: Value<Map<i32, (Value<i32>, Value<Item>)>> =
                Rc::new(RefCell::new(Map::new()));
            let keys: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
            if (hit != 0) {
                'loop_: for k in IterRange::new(keys.decay(), keys.decay().offset((3_i64) as isize))
                {
                    let k: Value<i32> = Rc::new(RefCell::new((k.read())));
                    field!(Map::subscript(selected.as_pointer(), k.as_pointer()), value).write(42);
                }
            }
            let a: Value<i32> = Rc::new(RefCell::new(0));
            let b: Value<i32> = Rc::new(RefCell::new(0));
            let c: Value<i32> = Rc::new(RefCell::new(0));
            let d: Value<i32> = Rc::new(RefCell::new(0));
            let e: Value<i32> = Rc::new(RefCell::new(0));
            let f: Value<i32> = Rc::new(RefCell::new(0));
            let prefix: Value<(
                Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
                Value<bool>,
            )> = Rc::new(RefCell::new(
                match (if (choose != 0) {
                    {
                        let __tmp_62 = a.as_pointer();
                        __tmp_62.with_mut(|__v| __v.prefix_inc());
                        __tmp_62
                    }
                } else {
                    {
                        let __tmp_63 = b.as_pointer();
                        __tmp_63.with_mut(|__v| __v.prefix_inc());
                        __tmp_63
                    }
                },)
                {
                    (__tmp_61,) => {
                        let (__it, __inserted) =
                            Map::emplace(selected.as_pointer(), &keys.decay().offset(0), || {
                                (
                                    Rc::new(RefCell::new(keys.decay().offset(0).read())),
                                    Rc::new(RefCell::new(Item::new_2({ (__tmp_61).clone() }, {
                                        2
                                    }))),
                                )
                            });
                        (
                            Rc::new(RefCell::new(__it)),
                            Rc::new(RefCell::new(__inserted)),
                        )
                    }
                },
            ));
            let assignment: Value<(
                Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
                Value<bool>,
            )> = Rc::new(RefCell::new(
                match (if (choose != 0) {
                    (match ({ 5 }, c.as_pointer()) {
                        (__tmp_66, __tmp_65) => {
                            __tmp_65.write(__tmp_66);
                            __tmp_65
                        }
                    })
                } else {
                    (match ({ 6 }, d.as_pointer()) {
                        (__tmp_68, __tmp_67) => {
                            __tmp_67.write(__tmp_68);
                            __tmp_67
                        }
                    })
                },)
                {
                    (__tmp_64,) => {
                        let (__it, __inserted) =
                            Map::emplace(selected.as_pointer(), &keys.decay().offset(1), || {
                                (
                                    Rc::new(RefCell::new(keys.decay().offset(1).read())),
                                    Rc::new(RefCell::new(Item::new_2({ (__tmp_64).clone() }, {
                                        2
                                    }))),
                                )
                            });
                        (
                            Rc::new(RefCell::new(__it)),
                            Rc::new(RefCell::new(__inserted)),
                        )
                    }
                },
            ));
            let compound: Value<(
                Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
                Value<bool>,
            )> = Rc::new(RefCell::new(
                match (if (choose != 0) {
                    (match ({ 1 }, e.as_pointer()) {
                        (__tmp_71, __tmp_70) => {
                            __tmp_70.with_mut(|__v| {
                                *__v = (((*__v) as i32) + (__tmp_71 as i32)) as i32
                            });
                            __tmp_70
                        }
                    })
                } else {
                    (match ({ 2 }, f.as_pointer()) {
                        (__tmp_73, __tmp_72) => {
                            __tmp_72.with_mut(|__v| {
                                *__v = (((*__v) as i32) + (__tmp_73 as i32)) as i32
                            });
                            __tmp_72
                        }
                    })
                },)
                {
                    (__tmp_69,) => {
                        let (__it, __inserted) =
                            Map::emplace(selected.as_pointer(), &keys.decay().offset(2), || {
                                (
                                    Rc::new(RefCell::new(keys.decay().offset(2).read())),
                                    Rc::new(RefCell::new(Item::new_2({ (__tmp_69).clone() }, {
                                        2
                                    }))),
                                )
                            });
                        (
                            Rc::new(RefCell::new(__it)),
                            Rc::new(RefCell::new(__inserted)),
                        )
                    }
                },
            ));
            assert!(
                ({
                    ((*a.borrow())
                        == (if (choose != 0) {
                            (if (hit != 0) { 1 } else { 3 })
                        } else {
                            0
                        }))
                        && ((*b.borrow())
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 1 } else { 3 })
                            }))
                })
            );
            assert!(
                ({
                    ((*c.borrow())
                        == (if (choose != 0) {
                            (if (hit != 0) { 5 } else { 7 })
                        } else {
                            0
                        }))
                        && ((*d.borrow())
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 6 } else { 8 })
                            }))
                })
            );
            assert!(
                ({
                    ((*e.borrow())
                        == (if (choose != 0) {
                            (if (hit != 0) { 1 } else { 3 })
                        } else {
                            0
                        }))
                        && ((*f.borrow())
                            == (if (choose != 0) {
                                0
                            } else {
                                (if (hit != 0) { 2 } else { 4 })
                            }))
                })
            );
            assert!(
                ({
                    ((((*(*prefix.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
                        && (((*(*assignment.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                        && (((*(*compound.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
                })
            );
            assert!(
                ({
                    ((*(*prefix.borrow()).0.borrow())
                        .iter_deref()
                        .with(|__s| (*({ __s }).1.borrow()).value)
                        == (if (hit != 0) { 42 } else { 3 }))
                })
            );
            assert!(
                ({
                    ((*(*assignment.borrow()).0.borrow())
                        .iter_deref()
                        .with(|__s| (*({ __s }).1.borrow()).value)
                        == (if (hit != 0) {
                            42
                        } else {
                            if (choose != 0) { 7 } else { 8 }
                        }))
                })
            );
            assert!(
                ({
                    ((*(*compound.borrow()).0.borrow())
                        .iter_deref()
                        .with(|__s| (*({ __s }).1.borrow()).value)
                        == (if (hit != 0) {
                            42
                        } else {
                            if (choose != 0) { 3 } else { 4 }
                        }))
                })
            );
            choose.prefix_inc();
        }
        let values: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        let keys: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        if (hit != 0) {
            'loop_: for k in IterRange::new(keys.decay(), keys.decay().offset((3_i64) as isize)) {
                let k: Value<i32> = Rc::new(RefCell::new((k.read())));
                Map::subscript(values.as_pointer(), k.as_pointer()).write(42);
            }
        }
        let mut before: i32 = temporary_calls_4.with(|rc| *rc.borrow());
        let mut index: i32 = 0;
        let member: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let __tmp_76: Value<Temporary>;
            match ((match {
                __tmp_76 = Rc::new(RefCell::new(({ make_temporary_5() })));
                __tmp_76.as_pointer()
            } {
                __tmp_75 => field_ptr!(__tmp_75, n),
            }),)
            {
                (__tmp_74,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(0), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(0).read())),
                                Rc::new(RefCell::new((__tmp_74.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let nested: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let __tmp_79: Value<Nested>;
            match ((match {
                __tmp_79 = Rc::new(RefCell::new(({ make_nested_6() })));
                __tmp_79.as_pointer()
            } {
                __tmp_78 => field_ptr!(field_ptr!(__tmp_78, inner), n),
            }),)
            {
                (__tmp_77,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(1), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(1).read())),
                                Rc::new(RefCell::new((__tmp_77.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let element: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let __tmp_82: Value<Temporary>;
            match ((match {
                __tmp_82 = Rc::new(RefCell::new(({ make_temporary_5() })));
                __tmp_82.as_pointer()
            } {
                __tmp_81 => array_field_ptr!(__tmp_81, data),
            })
            .offset((index.postfix_inc()) as isize),)
            {
                (__tmp_80,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(2), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(2).read())),
                                Rc::new(RefCell::new((__tmp_80.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        assert!((temporary_calls_4.with(|rc| *rc.borrow()) == (before + 3)) && (index == 1));
        assert!(
            ({
                ((((*(*member.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
                    && (((*(*nested.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*element.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
            })
        );
        assert!(
            ({
                (((*(*(*(*member.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                    == (if (hit != 0) { 42 } else { 5 }))
                    && ((*(*(*(*nested.borrow()).0.borrow())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .borrow())
                        == (if (hit != 0) { 42 } else { 5 })))
                    && ((*(*(*(*element.borrow()).0.borrow())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .borrow())
                        == (if (hit != 0) { 42 } else { 6 }))
            })
        );
        let mut choose: i32 = 0;
        'loop_: while (choose < 2) {
            let references: Value<Map<i32, (Value<i32>, Value<Held>)>> =
                Rc::new(RefCell::new(Map::new()));
            let ordinary: Value<i32> = Rc::new(RefCell::new(9));
            if (hit != 0) {
                {
                    let (__it, __inserted) =
                        Map::emplace(references.as_pointer(), &key.as_pointer(), || {
                            (
                                Rc::new(RefCell::new(key.as_pointer().read())),
                                Rc::new(RefCell::new(Held::new_1({ ordinary.as_pointer() }))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                };
            }
            before = temporary_calls_4.with(|rc| *rc.borrow());
            index = 0;
            let mut value: i32 = 0;
            {
                let __tmp_85: Value<Nested>;
                let __tmp_87: Value<Temporary>;
                {
                    match (if (choose != 0) {
                        (match {
                            __tmp_85 = Rc::new(RefCell::new(({ make_nested_6() })));
                            __tmp_85.as_pointer()
                        } {
                            __tmp_84 => field_ptr!(field_ptr!(__tmp_84, inner), n),
                        })
                    } else {
                        (match {
                            __tmp_87 = Rc::new(RefCell::new(({ make_temporary_5() })));
                            __tmp_87.as_pointer()
                        } {
                            __tmp_86 => array_field_ptr!(__tmp_86, data),
                        })
                        .offset((index.postfix_inc()) as isize)
                    },)
                    {
                        (__tmp_83,) => {
                            let (__it, __inserted) =
                                Map::emplace(references.as_pointer(), &key.as_pointer(), || {
                                    (
                                        Rc::new(RefCell::new(key.as_pointer().read())),
                                        Rc::new(RefCell::new(Held::new_1({ (__tmp_83).clone() }))),
                                    )
                                });
                            (
                                Rc::new(RefCell::new(__it)),
                                Rc::new(RefCell::new(__inserted)),
                            )
                        }
                    };
                    let __rhs = (Map::subscript(references.as_pointer(), key.as_pointer())
                        .with(|__s| __s.ptr.clone())
                        .read());
                    value = __rhs
                }
            };
            assert!(
                (temporary_calls_4.with(|rc| *rc.borrow()) == (before + 1))
                    && (index == (if (choose != 0) { 0 } else { 1 }))
            );
            assert!(
                (value
                    == (if (hit != 0) {
                        9
                    } else {
                        if (choose != 0) { 5 } else { 6 }
                    }))
            );
            let arrays: Value<Map<i32, (Value<i32>, Value<Held>)>> =
                Rc::new(RefCell::new(Map::new()));
            let ordinary_array: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([9, 10])));
            if (hit != 0) {
                {
                    let (__it, __inserted) =
                        Map::emplace(arrays.as_pointer(), &key.as_pointer(), || {
                            (
                                Rc::new(RefCell::new(key.as_pointer().read())),
                                Rc::new(RefCell::new(Held::new_3({ ordinary_array.decay() }, {
                                    0
                                }))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                };
            }
            before = temporary_calls_4.with(|rc| *rc.borrow());
            {
                let __tmp_90: Value<Nested>;
                let __tmp_92: Value<Temporary>;
                {
                    match (if (choose != 0) {
                        (match {
                            __tmp_90 = Rc::new(RefCell::new(({ make_nested_6() })));
                            __tmp_90.as_pointer()
                        } {
                            __tmp_89 => array_field_ptr!(field_ptr!(__tmp_89, inner), data),
                        })
                    } else {
                        (match {
                            __tmp_92 = Rc::new(RefCell::new(({ make_temporary_5() })));
                            __tmp_92.as_pointer()
                        } {
                            __tmp_91 => array_field_ptr!(__tmp_91, data),
                        })
                    },)
                    {
                        (__tmp_88,) => {
                            let (__it, __inserted) =
                                Map::emplace(arrays.as_pointer(), &key.as_pointer(), || {
                                    (
                                        Rc::new(RefCell::new(key.as_pointer().read())),
                                        Rc::new(RefCell::new(Held::new_3(
                                            { (__tmp_88).clone() },
                                            { 0 },
                                        ))),
                                    )
                                });
                            (
                                Rc::new(RefCell::new(__it)),
                                Rc::new(RefCell::new(__inserted)),
                            )
                        }
                    };
                    let __rhs = (elem!(
                        Map::subscript(arrays.as_pointer(), key.as_pointer())
                            .with(|__s| __s.ptr.clone()),
                        1
                    )
                    .read());
                    value = __rhs
                }
            };
            assert!(
                (temporary_calls_4.with(|rc| *rc.borrow()) == (before + 1))
                    && (value == (if (hit != 0) { 10 } else { 7 }))
            );
            choose.prefix_inc();
        }
        hit.prefix_inc();
    }
    let mut hit: i32 = 0;
    'loop_: while (hit < 2) {
        let values: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
        let keys: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5])));
        if (hit != 0) {
            'loop_: for k in IterRange::new(keys.decay(), keys.decay().offset((6_i64) as isize)) {
                let k: Value<i32> = Rc::new(RefCell::new((k.read())));
                Map::subscript(values.as_pointer(), k.as_pointer()).write(42);
            }
        }
        let mut before: i32 = temporary_calls_4.with(|rc| *rc.borrow());
        let mut index: i32 = 0;
        let arrow: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let mut __tmp_94: Option<Value<Temporary>>;
            match (field_ptr!(
                (*{
                    __tmp_94 = Some(Rc::new(RefCell::new(Temporary::new())));
                    &mut __tmp_94
                })
                .as_pointer(),
                n
            ),)
            {
                (__tmp_93,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(0), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(0).read())),
                                Rc::new(RefCell::new((__tmp_93.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let nested: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let mut __tmp_96: Option<Value<Nested>>;
            match (field_ptr!(
                field_ptr!(
                    (*{
                        __tmp_96 = Some(Rc::new(RefCell::new(<Nested>::default())));
                        &mut __tmp_96
                    })
                    .as_pointer(),
                    inner
                ),
                n
            ),)
            {
                (__tmp_95,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(1), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(1).read())),
                                Rc::new(RefCell::new((__tmp_95.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let element: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let mut __tmp_98: Option<Value<Temporary>>;
            match (array_field_ptr!(
                (*{
                    __tmp_98 = Some(Rc::new(RefCell::new(Temporary::new())));
                    &mut __tmp_98
                })
                .as_pointer(),
                data
            )
            .offset((index.postfix_inc()) as isize),)
            {
                (__tmp_97,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(2), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(2).read())),
                                Rc::new(RefCell::new((__tmp_97.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let get: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let mut __tmp_100: Option<Value<Temporary>>;
            match (field_ptr!(
                (*{
                    __tmp_100 = Some(Rc::new(RefCell::new(Temporary::new())));
                    &mut __tmp_100
                })
                .as_pointer(),
                n
            ),)
            {
                (__tmp_99,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(3), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(3).read())),
                                Rc::new(RefCell::new((__tmp_99.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let deref: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let mut __tmp_102: Option<Value<Temporary>>;
            match (field_ptr!(
                ((*{
                    __tmp_102 = Some(Rc::new(RefCell::new(Temporary::new())));
                    &mut __tmp_102
                })
                .as_pointer()),
                n
            ),)
            {
                (__tmp_101,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(4), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(4).read())),
                                Rc::new(RefCell::new((__tmp_101.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        let indirect: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new({
            let mut __tmp_104: Option<Value<Option<Value<Temporary>>>>;
            match (field_ptr!(
                ((*(*{
                    __tmp_104 = Some(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                        Temporary::new(),
                    ))))));
                    &mut __tmp_104
                })
                .as_pointer()
                .upgrade()
                .deref())
                .as_pointer()),
                n
            ),)
            {
                (__tmp_103,) => {
                    let (__it, __inserted) =
                        Map::emplace(values.as_pointer(), &keys.decay().offset(5), || {
                            (
                                Rc::new(RefCell::new(keys.decay().offset(5).read())),
                                Rc::new(RefCell::new((__tmp_103.read()))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                }
            }
        }));
        assert!((temporary_calls_4.with(|rc| *rc.borrow()) == (before + 6)) && (index == 1));
        assert!(
            ({
                (((((((*(*arrow.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
                    && (((*(*nested.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*element.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*get.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*deref.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32)))
                    && (((*(*indirect.borrow()).1.borrow()) as i32) == (!(hit != 0) as i32))
            })
        );
        'loop_: for k in IterRange::new(keys.decay(), keys.decay().offset((6_i64) as isize)) {
            let k: Value<i32> = Rc::new(RefCell::new((k.read())));
            assert!(
                ({
                    ((Map::subscript(values.as_pointer(), k.as_pointer()).read())
                        == (if (hit != 0) {
                            42
                        } else {
                            if ((*k.borrow()) == 2) { 6 } else { 5 }
                        }))
                })
            );
        }
        let mut choose: i32 = 0;
        'loop_: while (choose < 2) {
            let references: Value<Map<i32, (Value<i32>, Value<Held>)>> =
                Rc::new(RefCell::new(Map::new()));
            let ordinary: Value<i32> = Rc::new(RefCell::new(9));
            if (hit != 0) {
                {
                    let (__it, __inserted) =
                        Map::emplace(references.as_pointer(), &key.as_pointer(), || {
                            (
                                Rc::new(RefCell::new(key.as_pointer().read())),
                                Rc::new(RefCell::new(Held::new_1({ ordinary.as_pointer() }))),
                            )
                        });
                    (
                        Rc::new(RefCell::new(__it)),
                        Rc::new(RefCell::new(__inserted)),
                    )
                };
            }
            before = temporary_calls_4.with(|rc| *rc.borrow());
            index = 0;
            let mut value: i32 = 0;
            {
                let mut __tmp_106: Option<Value<Temporary>>;
                let mut __tmp_107: Option<Value<Temporary>>;
                {
                    match (if (choose != 0) {
                        field_ptr!(
                            (*{
                                __tmp_106 = Some(Rc::new(RefCell::new(Temporary::new())));
                                &mut __tmp_106
                            })
                            .as_pointer(),
                            n
                        )
                    } else {
                        array_field_ptr!(
                            (*{
                                __tmp_107 = Some(Rc::new(RefCell::new(Temporary::new())));
                                &mut __tmp_107
                            })
                            .as_pointer(),
                            data
                        )
                        .offset((index.postfix_inc()) as isize)
                    },)
                    {
                        (__tmp_105,) => {
                            let (__it, __inserted) =
                                Map::emplace(references.as_pointer(), &key.as_pointer(), || {
                                    (
                                        Rc::new(RefCell::new(key.as_pointer().read())),
                                        Rc::new(RefCell::new(Held::new_1({ (__tmp_105).clone() }))),
                                    )
                                });
                            (
                                Rc::new(RefCell::new(__it)),
                                Rc::new(RefCell::new(__inserted)),
                            )
                        }
                    };
                    let __rhs = (Map::subscript(references.as_pointer(), key.as_pointer())
                        .with(|__s| __s.ptr.clone())
                        .read());
                    value = __rhs
                }
            };
            assert!(
                (temporary_calls_4.with(|rc| *rc.borrow()) == (before + 1))
                    && (index == (if (choose != 0) { 0 } else { 1 }))
            );
            assert!(
                (value
                    == (if (hit != 0) {
                        9
                    } else {
                        if (choose != 0) { 5 } else { 6 }
                    }))
            );
            let owners: Value<Map<i32, (Value<i32>, Value<Held>)>> =
                Rc::new(RefCell::new(Map::new()));
            {
                let __tmp_109: Value<Owner>;
                {
                    match (({
                        OwnerImpl::value(&{
                            __tmp_109 = Rc::new(RefCell::new(Owner::new()));
                            __tmp_109.as_pointer()
                        })
                    }),)
                    {
                        (__tmp_108,) => {
                            let (__it, __inserted) =
                                Map::emplace(owners.as_pointer(), &key.as_pointer(), || {
                                    (
                                        Rc::new(RefCell::new(key.as_pointer().read())),
                                        Rc::new(RefCell::new(Held::new_1({ (__tmp_108).clone() }))),
                                    )
                                });
                            (
                                Rc::new(RefCell::new(__it)),
                                Rc::new(RefCell::new(__inserted)),
                            )
                        }
                    };
                    let __rhs = (Map::subscript(owners.as_pointer(), key.as_pointer())
                        .with(|__s| __s.ptr.clone())
                        .read());
                    value = __rhs
                }
            };
            assert!((value == 5));
            choose.prefix_inc();
        }
        hit.prefix_inc();
    }
    let move_only: Value<
        Map<
            i32,
            (
                Value<i32>,
                Value<(Value<Option<Value<Noncopy>>>, Value<i32>)>,
            ),
        >,
    > = Rc::new(RefCell::new(Map::new()));
    let move_new: Value<(
        Value<
            BidirectionalIterator<(
                Value<i32>,
                Value<(Value<Option<Value<Noncopy>>>, Value<i32>)>,
            )>,
        >,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __tmp_111: Value<Option<Value<Noncopy>>>;
        match ({
            __tmp_111 = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                <Noncopy>::default(),
            )))));
            __tmp_111.as_pointer()
        },)
        {
            (__tmp_110,) => {
                let (__it, __inserted) =
                    Map::emplace(move_only.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new((
                                Rc::new(RefCell::new(
                                    __tmp_110
                                        .with_mut(|__v: &mut Option<Value<Noncopy>>| __v.take()),
                                )),
                                Rc::new(RefCell::new(7)),
                            ))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        }
    }));
    let move_hit: Value<(
        Value<
            BidirectionalIterator<(
                Value<i32>,
                Value<(Value<Option<Value<Noncopy>>>, Value<i32>)>,
            )>,
        >,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __tmp_113: Value<Option<Value<Noncopy>>>;
        match ({
            __tmp_113 = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                <Noncopy>::default(),
            )))));
            __tmp_113.as_pointer()
        },)
        {
            (__tmp_112,) => {
                let (__it, __inserted) =
                    Map::emplace(move_only.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new((
                                Rc::new(RefCell::new(
                                    __tmp_112
                                        .with_mut(|__v: &mut Option<Value<Noncopy>>| __v.take()),
                                )),
                                Rc::new(RefCell::new(9)),
                            ))),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        }
    }));
    assert!(({ (*(*move_new.borrow()).1.borrow()) && (!(*(*move_hit.borrow()).1.borrow())) }));
    assert!(
        ({
            ((*(*(*(*(*move_new.borrow()).0.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
            .0
            .borrow())
            .as_pointer()
            .with(|__s| __s.n)
                == 5)
                && ((*(*(*(*(*move_hit.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .borrow())
                .1
                .borrow())
                    == 7)
        })
    );
    let strings: Value<Map<i32, (Value<i32>, Value<Vec<i8>>)>> = Rc::new(RefCell::new(Map::new()));
    let text: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"value").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    {
        let (__it, __inserted) = Map::emplace(strings.as_pointer(), &key.as_pointer(), || {
            (
                Rc::new(RefCell::new(key.as_pointer().read())),
                Rc::new(RefCell::new({
                    let mut __bytes = Ptr::<i8>::from_string_literal(b"original").to_c_bytes();
                    __bytes.push(0);
                    __bytes
                })),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    let string_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Vec<i8>>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match ({
            (*visits_1.with(Value::clone).borrow_mut()).prefix_inc();
            text.as_pointer()
        },)
        {
            (__tmp_114,) => {
                let (__it, __inserted) =
                    Map::emplace(strings.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(
                                __tmp_114.with_mut(|__v| std::mem::replace(__v, vec![0])),
                            )),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ((visits_1.with(|rc| *rc.borrow()) == 3) && (!(*(*string_hit.borrow()).1.borrow())))
                && (Ptr::<i8>::from_string_literal(b"value").with_c_str(|__s| {
                    (*text.borrow())[..(*text.borrow()).len().saturating_sub(1)] == *__s
                }))
        })
    );
    (*key.borrow_mut()).prefix_inc();
    let string_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Vec<i8>>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match ({
            (*visits_1.with(Value::clone).borrow_mut()).prefix_inc();
            text.as_pointer()
        },)
        {
            (__tmp_115,) => {
                let (__it, __inserted) =
                    Map::emplace(strings.as_pointer(), &key.as_pointer(), || {
                        (
                            Rc::new(RefCell::new(key.as_pointer().read())),
                            Rc::new(RefCell::new(
                                __tmp_115.with_mut(|__v| std::mem::replace(__v, vec![0])),
                            )),
                        )
                    });
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ((visits_1.with(|rc| *rc.borrow()) == 4) && (*(*string_new.borrow()).1.borrow()))
                && (match ((*(*(*string_new.borrow()).0.borrow())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .1
                .as_pointer(),)
                {
                    (__tmp_116,) => Ptr::<i8>::from_string_literal(b"value").with_c_str(|__s| {
                        (*__tmp_116.upgrade().deref())
                            [..(*__tmp_116.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                })
        })
    );
    return 0;
}
pub trait OwnerImpl {
    fn value(&self) -> Ptr<i32>;
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn value(&self) -> Ptr<i32> {
        return field_ptr!((*self).with(|__s| __s.ptr.clone()).as_pointer(), n);
    }
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        field_ptr!((*self), ptr)
            .write(field!(_a0, ptr).with_mut(|__v: &mut Option<Value<Temporary>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = made_0.with(|_| ());
    let _ = visits_1.with(|_| ());
    let _ = temporary_calls_4.with(|_| ());
    let _ = global_values_7.with(|_| ());
    let _ = global_key_8.with(|_| ());
    let _ = conditional_key_9.with(|_| ());
    let _ = owned_key_10.with(|_| ());
    let _ = global_a_11.with(|_| ());
    let _ = global_b_12.with(|_| ());
    let _ = global_choose_13.with(|_| ());
    let _ = global_temporary_14.with(|_| ());
    let _ = global_conditional_15.with(|_| ());
    let _ = global_owned_16.with(|_| ());
}
