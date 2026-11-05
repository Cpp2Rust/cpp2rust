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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
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
            value: {
                {
                    let __rhs = { amount };
                    counter.with_mut(|__v| *__v = *__v + __rhs)
                };
                (counter.read())
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
        Map::subscript(
            global_map_6.with(|v| v.as_pointer()),
            global_key_7.with(|v| v.as_pointer()),
        )
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
            let __tmp_1 = global_counter_8.with(|v| v.as_pointer());
            __tmp_1.with_mut(|__v| __v.prefix_inc());
            __tmp_1
        },)
        {
            (__tmp_0,) => {
                let __inserted = global_map_6.with(|v| v.as_pointer()).with(
                    |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.get(&global_key_7.with(|v| v.as_pointer())).is_none()
                    },
                );
                if __inserted {
                    Map::subscript(
                        global_map_6.with(|v| v.as_pointer()).clone(),
                        global_key_7.with(|v| v.as_pointer()).clone(),
                    )
                    .write((__tmp_0.read()));
                }
                (
                    Rc::new(RefCell::new(global_map_6.with(|v| v.as_pointer()).with(
                        |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                            __m.find(&global_key_7.with(|v| v.as_pointer()))
                        },
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut ok: bool = ((global_counter_8.with(|rc| *rc.borrow()) == 1)
        && (!(*(*global_hit_10.with(Value::clone).borrow()).1.borrow())))
        && ((*(*(*(*global_hit_10.with(Value::clone).borrow()).0.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
            == 10);
    let mut rows: i32 = 0;
    let mut subset: i32 = 0;
    'loop_: while (subset < 8) {
        let key: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*key.borrow()) < 3) {
            let mut initial: i32 = 0;
            'loop_: while (initial < 3) {
                let mut post: i32 = 0;
                'loop_: while (post < 2) {
                    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> =
                        Rc::new(RefCell::new(Map::new()));
                    let mut size: i32 = 0;
                    let k: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*k.borrow()) < 3) {
                        if ((subset & (1 << (*k.borrow()))) != 0) {
                            let __rhs = (10 + (*k.borrow()));
                            Map::subscript(m.as_pointer(), k.as_pointer()).write(__rhs);
                            size.prefix_inc();
                        }
                        (*k.borrow_mut()).prefix_inc();
                    }
                    let counter: Value<i32> = Rc::new(RefCell::new(initial));
                    let result: Value<(
                        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
                        Value<bool>,
                    )> = Rc::new(RefCell::new(if (post != 0) {
                        match (Rc::new(RefCell::new((*counter.borrow_mut()).postfix_inc()))
                            .as_pointer(),)
                        {
                            (__tmp_2,) => {
                                let __inserted = m.as_pointer().with(
                                    |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                                        __m.get(&key.as_pointer()).is_none()
                                    },
                                );
                                if __inserted {
                                    Map::subscript(
                                        m.as_pointer().clone(),
                                        key.as_pointer().clone(),
                                    )
                                    .write((__tmp_2.read()));
                                }
                                (
                                    Rc::new(RefCell::new(m.as_pointer().with(
                                        |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                                            __m.find(&key.as_pointer())
                                        },
                                    ))),
                                    Rc::new(RefCell::new(__inserted)),
                                )
                            }
                        }
                    } else {
                        match ({
                            let __tmp_4 = counter.as_pointer();
                            __tmp_4.with_mut(|__v| __v.prefix_inc());
                            __tmp_4
                        },)
                        {
                            (__tmp_3,) => {
                                let __inserted = m.as_pointer().with(
                                    |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                                        __m.get(&key.as_pointer()).is_none()
                                    },
                                );
                                if __inserted {
                                    Map::subscript(
                                        m.as_pointer().clone(),
                                        key.as_pointer().clone(),
                                    )
                                    .write((__tmp_3.read()));
                                }
                                (
                                    Rc::new(RefCell::new(m.as_pointer().with(
                                        |__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                                            __m.find(&key.as_pointer())
                                        },
                                    ))),
                                    Rc::new(RefCell::new(__inserted)),
                                )
                            }
                        }
                    }));
                    let mut hit: bool = ((subset & (1 << (*key.borrow()))) != 0);
                    ok = { ((ok as i32) & (((*counter.borrow()) == (initial + 1)) as i32)) != 0 };
                    ok = {
                        ((ok as i32)
                            & ((((*(*result.borrow()).1.borrow()) as i32) == (!(hit) as i32))
                                as i32))
                            != 0
                    };
                    let mut found: bool = (*(*result.borrow()).0.borrow())
                        == m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                                __m.find(&key.as_pointer())
                            });
                    ok = { ((ok as i32) & (found as i32)) != 0 };
                    {
                        let rhs_0 = ((ok as i32)
                            & (((*(*(*(*result.borrow()).0.borrow())
                                .iter_deref()
                                .upgrade()
                                .deref())
                            .0
                            .borrow())
                                == (*key.borrow())) as i32))
                            != 0;
                        ok = rhs_0
                    };
                    {
                        let rhs_0 = ((ok as i32)
                            & (((*(*(*(*result.borrow()).0.borrow())
                                .iter_deref()
                                .upgrade()
                                .deref())
                            .1
                            .borrow())
                                == (if hit {
                                    (10 + (*key.borrow()))
                                } else {
                                    (initial + (!(post != 0) as i32))
                                })) as i32))
                            != 0;
                        ok = rhs_0
                    };
                    {
                        let rhs_0 = ((ok as i32)
                            & (((*m.borrow()).len() == ((size + (!(hit) as i32)) as usize))
                                as i32))
                            != 0;
                        ok = rhs_0
                    };
                    rows.prefix_inc();
                    post.prefix_inc();
                }
                initial.prefix_inc();
            }
            (*key.borrow_mut()).prefix_inc();
        }
        subset.prefix_inc();
    }
    let items: Value<Map<i32, (Value<i32>, Value<Item>)>> = Rc::new(RefCell::new(Map::new()));
    let key: Value<i32> = Rc::new(RefCell::new(0));
    {
        let __inserted = items
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.get(&key.as_pointer()).is_none());
        if __inserted {
            Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                .write(Item::new_1({ 10 }));
        }
        (
            Rc::new(RefCell::new(items.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    let source: Value<Item> = Rc::new(RefCell::new(Item::new_1({ 20 })));
    made_1.with(|rc| {
        *rc.borrow_mut() = {
            copies_2.with(|rc| {
                *rc.borrow_mut() = {
                    moves_3.with(|rc| *rc.borrow_mut() = 0);
                    moves_3.with(|rc| *rc.borrow())
                }
            });
            copies_2.with(|rc| *rc.borrow())
        }
    });
    let hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_5,) => {
                let __inserted =
                    items
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::copy_from({ (__tmp_5).clone() }));
                }
                (
                    Rc::new(RefCell::new(items.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((calls_0.with(|rc| *rc.borrow()) == 1) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 0)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((!(*(*hit.borrow()).1.borrow()))
                && ((*(*hit.borrow()).0.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 10)) as i32))
            != 0;
        ok = rhs_0
    };
    let move_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_6,) => {
                let __inserted =
                    items
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::move_from({ (__tmp_6).clone() }));
                }
                (
                    Rc::new(RefCell::new(items.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((calls_0.with(|rc| *rc.borrow()) == 2) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 0)) as i32))
            != 0
    };
    ok = {
        ((ok as i32)
            & (((!(*(*move_hit.borrow()).1.borrow())) && ({ (*source.borrow()).value } == 20))
                as i32))
            != 0
    };
    (*key.borrow_mut()).prefix_inc();
    let copied: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_7,) => {
                let __inserted =
                    items
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::copy_from({ (__tmp_7).clone() }));
                }
                (
                    Rc::new(RefCell::new(items.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((calls_0.with(|rc| *rc.borrow()) == 3) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 1))
                && (moves_3.with(|rc| *rc.borrow()) == 0)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*(*copied.borrow()).1.borrow())
                && ((*(*copied.borrow()).0.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 20)) as i32))
            != 0;
        ok = rhs_0
    };
    (*key.borrow_mut()).prefix_inc();
    let moved: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ argument_4(source.as_pointer()) }),) {
            (__tmp_8,) => {
                let __inserted =
                    items
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::move_from({ (__tmp_8).clone() }));
                }
                (
                    Rc::new(RefCell::new(items.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((calls_0.with(|rc| *rc.borrow()) == 4) && (made_1.with(|rc| *rc.borrow()) == 0))
                && (copies_2.with(|rc| *rc.borrow()) == 1))
                && (moves_3.with(|rc| *rc.borrow()) == 1)) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & ((((*(*moved.borrow()).1.borrow())
                && ((*(*moved.borrow()).0.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 20))
                && ({ (*source.borrow()).value } == -1_i32)) as i32))
            != 0;
        ok = rhs_0
    };
    let refs: Value<Map<i32, (Value<i32>, Value<RefItem>)>> = Rc::new(RefCell::new(Map::new()));
    (*key.borrow_mut()) = 0;
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    {
        let __inserted = refs
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<RefItem>)>| {
                __m.get(&key.as_pointer()).is_none()
            });
        if __inserted {
            Map::subscript(refs.as_pointer().clone(), key.as_pointer().clone())
                .write(RefItem::new_2({ counter.as_pointer() }, { 5 }));
        }
        (
            Rc::new(RefCell::new(refs.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<RefItem>)>| __m.find(&key.as_pointer()),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    made_1.with(|rc| *rc.borrow_mut() = 0);
    let ref_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<RefItem>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ reference_5(counter.as_pointer()) }),) {
            (__tmp_9,) => {
                let __inserted =
                    refs.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<RefItem>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(refs.as_pointer().clone(), key.as_pointer().clone())
                        .write(RefItem::new_2({ (__tmp_9).clone() }, { 2 }));
                }
                (
                    Rc::new(RefCell::new(refs.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<RefItem>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((calls_0.with(|rc| *rc.borrow()) == 5) && ((*counter.borrow()) == 5))
                && (made_1.with(|rc| *rc.borrow()) == 0))
                && (!(*(*ref_hit.borrow()).1.borrow()))) as i32))
            != 0
    };
    (*key.borrow_mut()).prefix_inc();
    let ref_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<RefItem>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (({ reference_5(counter.as_pointer()) }),) {
            (__tmp_10,) => {
                let __inserted =
                    refs.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<RefItem>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(refs.as_pointer().clone(), key.as_pointer().clone())
                        .write(RefItem::new_2({ (__tmp_10).clone() }, { 2 }));
                }
                (
                    Rc::new(RefCell::new(refs.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<RefItem>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((calls_0.with(|rc| *rc.borrow()) == 6) && ((*counter.borrow()) == 7))
                && (made_1.with(|rc| *rc.borrow()) == 1))
                && (*(*ref_new.borrow()).1.borrow())) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & ((((*(*ref_hit.borrow()).0.borrow())
                .iter_deref()
                .with(|__s| (*({ __s }).1.borrow()).value)
                == 5)
                && ((*(*ref_new.borrow()).0.borrow())
                    .iter_deref()
                    .with(|__s| (*({ __s }).1.borrow()).value)
                    == 7)) as i32))
            != 0;
        ok = rhs_0
    };
    (*key.borrow_mut()) = 0;
    made_1.with(|rc| {
        *rc.borrow_mut() = {
            copies_2.with(|rc| {
                *rc.borrow_mut() = {
                    moves_3.with(|rc| *rc.borrow_mut() = 0);
                    moves_3.with(|rc| *rc.borrow())
                }
            });
            copies_2.with(|rc| *rc.borrow())
        }
    });
    let temporary_hit: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new(Item::new_1({ 30 }))).as_pointer(),) {
            (__tmp_11,) => {
                let __inserted =
                    items
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::move_from({ (__tmp_11).clone() }));
                }
                (
                    Rc::new(RefCell::new(items.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((made_1.with(|rc| *rc.borrow()) == 1)
                && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 0))
                && (!(*(*temporary_hit.borrow()).1.borrow()))) as i32))
            != 0
    };
    (*key.borrow_mut()).prefix_inc();
    (*key.borrow_mut()).prefix_inc();
    (*key.borrow_mut()).prefix_inc();
    let temporary_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Item>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new(Item::new_1({ 40 }))).as_pointer(),) {
            (__tmp_12,) => {
                let __inserted =
                    items
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Item>)>| {
                            __m.get(&key.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(items.as_pointer().clone(), key.as_pointer().clone())
                        .write(Item::move_from({ (__tmp_12).clone() }));
                }
                (
                    Rc::new(RefCell::new(items.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Item>)>| __m.find(&key.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            }
        },
    ));
    ok = {
        ((ok as i32)
            & (((((made_1.with(|rc| *rc.borrow()) == 2)
                && (copies_2.with(|rc| *rc.borrow()) == 0))
                && (moves_3.with(|rc| *rc.borrow()) == 1))
                && (*(*temporary_new.borrow()).1.borrow())) as i32))
            != 0
    };
    {
        let rhs_0 = ((ok as i32)
            & (((*(*temporary_new.borrow()).0.borrow())
                .iter_deref()
                .with(|__s| (*({ __s }).1.borrow()).value)
                == 40) as i32))
            != 0;
        ok = rhs_0
    };
    println!(
        "{} {} {} {} {}",
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
