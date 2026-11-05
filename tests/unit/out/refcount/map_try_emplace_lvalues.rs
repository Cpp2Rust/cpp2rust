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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Map<i32, (Value<i32>, Value<Item>)>> = Rc::new(RefCell::new(Map::new()));
    let key: Value<i32> = Rc::new(RefCell::new(0));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    {
        let __inserted = m
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.get(&key.as_pointer()).is_none());
        if __inserted {
            Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                .write(Item::new_2({ counter.as_pointer() }, { 3 }));
        }
        (
            Rc::new(RefCell::new(m.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    made_0.with(|rc| *rc.borrow_mut() = 0);
    let hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ receiver_2(m.as_pointer()) }), {
            let __tmp_2 = ({ place_3(counter.as_pointer()) });
            __tmp_2.with_mut(|__v| __v.prefix_inc());
            __tmp_2
        }) {
            (__tmp_0, __tmp_1) => {
                let __inserted = __tmp_0.with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
                if __inserted {
                    Map::subscript(__tmp_0.clone(), key.as_pointer().clone())
                        .write(Item::new_2({ (__tmp_1).clone() }, { 2 }));
                }
                (
                    Rc::new(RefCell::new(__tmp_0.with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
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
            let __tmp_4 = counter.as_pointer();
            __tmp_4.with_mut(|__v| __v.prefix_inc());
            __tmp_4
        },)
        {
            (__tmp_3,) => {
                let __inserted =
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::new_2({ (__tmp_3).clone() }, { 2 }));
                }
                (
                    Rc::new(RefCell::new(m.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
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
            (__tmp_7, __tmp_6) => {
                __tmp_6.write({ __tmp_7 });
                __tmp_6
            }
        },)
        {
            (__tmp_5,) => {
                let __inserted =
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::new_2({ (__tmp_5).clone() }, { 2 }));
                }
                (
                    Rc::new(RefCell::new(m.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
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
            (__tmp_10, __tmp_9) => {
                {
                    let __rhs = { __tmp_10 };
                    __tmp_9.with_mut(|__v| *__v = *__v + __rhs)
                };
                __tmp_9
            }
        },)
        {
            (__tmp_8,) => {
                let __inserted =
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::new_2({ (__tmp_8).clone() }, { 2 }));
                }
                (
                    Rc::new(RefCell::new(m.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
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
        let __tmp_12 = (array.as_pointer() as Ptr<i32>).offset(index.postfix_inc());
        __tmp_12.with_mut(|__v| __v.prefix_inc());
        __tmp_12
    },)
    {
        (__tmp_11,) => {
            let __inserted = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
            if __inserted {
                Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                    .write(Item::new_2({ (__tmp_11).clone() }, { 3 }));
            }
            (
                Rc::new(RefCell::new(m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                ))),
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
    match (
        match (
            { 5 },
            (array.as_pointer() as Ptr<i32>).offset(index.postfix_inc()),
        ) {
            (__tmp_15, __tmp_14) => {
                __tmp_14.write({ __tmp_15 });
                __tmp_14
            }
        },
    ) {
        (__tmp_13,) => {
            let __inserted = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
            if __inserted {
                Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                    .write(Item::new_2({ (__tmp_13).clone() }, { 3 }));
            }
            (
                Rc::new(RefCell::new(m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                ))),
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
            (__tmp_18, __tmp_17) => {
                __tmp_17.write({ __tmp_18 });
                __tmp_17
            }
        },
    ) {
        (__tmp_16,) => {
            let __inserted = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
            if __inserted {
                Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                    .write(Item::new_2({ (__tmp_16).clone() }, { 3 }));
            }
            (
                Rc::new(RefCell::new(m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                ))),
                Rc::new(RefCell::new(__inserted)),
            )
        }
    };
    assert!(({ ((*counter.borrow()) == 16) && (made_0.with(|rc| *rc.borrow()) == 4) }));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    let b: Value<i32> = Rc::new(RefCell::new(0));
    let mut choose: bool = false;
    match (if choose {
        ({
            let __tmp_20 = a.as_pointer();
            __tmp_20.with_mut(|__v| __v.prefix_inc());
            __tmp_20
        })
    } else {
        ({
            let __tmp_21 = b.as_pointer();
            __tmp_21.with_mut(|__v| __v.prefix_inc());
            __tmp_21
        })
    },)
    {
        (__tmp_19,) => {
            let __inserted = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
            if __inserted {
                Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                    .write(Item::new_2({ (__tmp_19).clone() }, { 3 }));
            }
            (
                Rc::new(RefCell::new(m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                ))),
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
        ({
            let __tmp_23 = a.as_pointer();
            __tmp_23.with_mut(|__v| __v.prefix_inc());
            __tmp_23
        })
    } else {
        ({
            let __tmp_24 = b.as_pointer();
            __tmp_24.with_mut(|__v| __v.prefix_inc());
            __tmp_24
        })
    },)
    {
        (__tmp_22,) => {
            let __inserted = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
            if __inserted {
                Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                    .write(Item::new_2({ (__tmp_22).clone() }, { 3 }));
            }
            (
                Rc::new(RefCell::new(m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                ))),
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
        (__tmp_25,) => {
            let __inserted = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
            if __inserted {
                Map::subscript(m.as_pointer().clone(), key.as_pointer().clone())
                    .write(Item::new_2({ (__tmp_25).clone() }, { 3 }));
            }
            (
                Rc::new(RefCell::new(m.as_pointer().with(
                    |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                ))),
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
            (__tmp_28, __tmp_27) => {
                __tmp_27.write({ (__tmp_27.read()).wrapping_add(__tmp_28) });
                __tmp_27
            }
        },)
        {
            (__tmp_26,) => {
                let __inserted =
                    unsigned_map
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<u32>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(unsigned_map.as_pointer().clone(), key.as_pointer().clone())
                        .write((__tmp_26.read()));
                }
                (
                    Rc::new(RefCell::new(unsigned_map.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<u32>)>| __m.find(&key.as_pointer()),
                    ))),
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
        match (
            match (
                { 2 },
                (bytes.as_pointer() as Ptr<u8>).offset(index.postfix_inc()),
            ) {
                (__tmp_31, __tmp_30) => {
                    __tmp_30.write({ (((__tmp_30.read()) as i32) + __tmp_31) as u8 });
                    __tmp_30
                }
            },
        ) {
            (__tmp_29,) => {
                let __inserted =
                    unsigned_map
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<u32>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(unsigned_map.as_pointer().clone(), key.as_pointer().clone())
                        .write(((__tmp_29.read()) as u32));
                }
                (
                    Rc::new(RefCell::new(unsigned_map.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<u32>)>| __m.find(&key.as_pointer()),
                    ))),
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
    let pointers: Value<Map<i32, (Value<i32>, Value<Ptr<i32>>)>> =
        Rc::new(RefCell::new(Map::new()));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((array.as_pointer() as Ptr<i32>)));
    let pointer: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Ptr<i32>>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (match ({ 1 }, p.as_pointer()) {
            (__tmp_34, __tmp_33) => {
                let __rhs = __tmp_34;
                {
                    let _ptr = __tmp_33.clone();
                    _ptr.write(_ptr.read() + __rhs)
                };
                __tmp_33
            }
        },)
        {
            (__tmp_32,) => {
                let __inserted =
                    pointers
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Ptr<i32>>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(pointers.as_pointer().clone(), key.as_pointer().clone())
                        .write((__tmp_32.read()));
                }
                (
                    Rc::new(RefCell::new(pointers.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Ptr<i32>>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    assert!(
        ({
            ({ (*p.borrow()).clone() } == { (array.as_pointer() as Ptr<i32>).offset((1) as isize) })
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
    let strings: Value<Map<i32, (Value<i32>, Value<Vec<i8>>)>> = Rc::new(RefCell::new(Map::new()));
    let text: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"value").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    {
        let __inserted =
            strings
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Vec<i8>>)>| {
                    __m.get(&key.as_pointer()).is_none()
                });
        if __inserted {
            Map::subscript(strings.as_pointer().clone(), key.as_pointer().clone()).write({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"original").to_c_bytes();
                __bytes.push(0);
                __bytes
            });
        }
        (
            Rc::new(RefCell::new(strings.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Vec<i8>>)>| __m.find(&key.as_pointer()),
            ))),
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
            (__tmp_35,) => {
                let __inserted =
                    strings
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Vec<i8>>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(strings.as_pointer().clone(), key.as_pointer().clone())
                        .write(__tmp_35.with_mut(std::mem::take));
                }
                (
                    Rc::new(RefCell::new(strings.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Vec<i8>>)>| __m.find(&key.as_pointer()),
                    ))),
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
            (__tmp_36,) => {
                let __inserted =
                    strings
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Vec<i8>>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(strings.as_pointer().clone(), key.as_pointer().clone())
                        .write(__tmp_36.with_mut(std::mem::take));
                }
                (
                    Rc::new(RefCell::new(strings.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Vec<i8>>)>| __m.find(&key.as_pointer()),
                    ))),
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
                    (__tmp_37,) => Ptr::<i8>::from_string_literal(b"value").with_c_str(|__s| {
                        (*__tmp_37.upgrade().deref())
                            [..(*__tmp_37.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                })
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = made_0.with(|_| ());
    let _ = visits_1.with(|_| ());
}
