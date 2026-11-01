extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut x: u32) {
    x = { (x).wrapping_add(1_u32) };
}
pub fn bar_1(x: Ptr<u32>) {
    x.write({ (x.read()).wrapping_add(1_u32) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new_1() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn new_2(mut x: i32, mut y: i32) -> Self {
        Self { x: x, y: y }
    }
}
impl Default for Point {
    fn default() -> Self {
        { Point::new_1() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Map<i16, (Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(0_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(1_u32);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(2_u32);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(2_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(3_u32);
    assert!(({ ((*m.borrow()).len() == 3_usize) }));
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 1_u32)
    );
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(1_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 2_u32)
    );
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 3_u32)
    );
    let mut x: i32 = 4;
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write((x as u32));
    assert!(({ ((*m.borrow()).len() == 3_usize) }));
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 1_u32)
    );
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(1_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 4_u32)
    );
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 3_u32)
    );
    ({
        foo_0(
            (Map::subscript_move(
                m.as_pointer(),
                Rc::new(RefCell::new(0_i16)).as_pointer(),
                |__k| cpp_move!(__k),
            )
            .read()),
        )
    });
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 1_u32)
    );
    ({
        bar_1(Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k),
        ))
    });
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 4_u32)
    );
    let __rhs = (Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(0_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .read())
    .wrapping_add(
        (Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .read()),
    );
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(0_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(__rhs);
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 5_u32)
    );
    let end: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end()),
    ));
    let it: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<i16> = Rc::new(RefCell::new(1_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_0.as_pointer()))
        }),
    ));
    let const_it: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            ({
                let __tmp_1: Value<i16> = Rc::new(RefCell::new(10_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_1.as_pointer())
                    })
            }),
        ),
    ));
    let mut x1: u32 = if (*it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
    };
    assert!((x1 == 4_u32));
    let mut x2: u32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(({ (*end.borrow()).clone() }))
    {
        0_u32
    } else {
        (*(*(*const_it.borrow()).iter_deref().upgrade().deref())
            .1
            .borrow())
    };
    assert!((x2 == 0_u32));
    let mut x3: u32 = if (*it.borrow())
        == m.as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    {
        0_u32
    } else {
        (*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
    };
    assert!((x3 == 4_u32));
    let mut x4: u32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end()),
        ) {
        0_u32
    } else {
        (*(*(*const_it.borrow()).iter_deref().upgrade().deref())
            .1
            .borrow())
    };
    assert!((x4 == 0_u32));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(4_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(5_u32);
    let it4: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        ({
            let __tmp_2: Value<i16> = Rc::new(RefCell::new(4_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_2.as_pointer()))
        }),
    ));
    let mut p: Ptr<u32> = ((*(*it4.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    let mut x5: u32 = (p.read());
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(4_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 5_u32)
    );
    assert!(({ ((*(*(*it4.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 5_u32) }));
    assert!(((p.read()) == 5_u32));
    assert!((x5 == 5_u32));
    p.with_mut(|__v| __v.prefix_inc());
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(4_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 6_u32)
    );
    assert!(({ ((*(*(*it4.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 6_u32) }));
    assert!(((p.read()) == 6_u32));
    assert!((x5 == 5_u32));
    let const_it4: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            ({
                let __tmp_3: Value<i16> = Rc::new(RefCell::new(4_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_3.as_pointer())
                    })
            }),
        ),
    ));
    assert!(
        ({
            ((*(*(*const_it4.borrow()).iter_deref().upgrade().deref())
                .1
                .borrow())
                == 6_u32)
        })
    );
    let r: Ptr<Map<i16, (Value<i16>, Value<u32>)>> = m.as_pointer();
    assert!(({ ((*r.upgrade().deref()).len() == 4_usize) }));
    assert!(
        ({
            let __tmp_4: Value<i16> = Rc::new(RefCell::new(4_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_4.as_pointer()))
        }) != m
            .as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    Map::erase((r).clone(), &({ (*it4.borrow()).clone() }));
    assert!(({ ((*r.upgrade().deref()).len() == 3_usize) }));
    assert!(
        ({
            let __tmp_5: Value<i16> = Rc::new(RefCell::new(4_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_5.as_pointer()))
        }) == m
            .as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(7_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(7_u32);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(8_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(8_u32);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(9_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(9_u32);
    assert!(({ ((*r.upgrade().deref()).len() == 6_usize) }));
    assert!(
        (({
            let __tmp_6: Value<i16> = Rc::new(RefCell::new(10_i16));
            Map::erase_key(m.as_pointer(), &__tmp_6.as_pointer())
        }) == 0_usize)
    );
    let mut key9: i32 = 9;
    assert!(
        (({
            let __tmp_7: Value<i16> = Rc::new(RefCell::new((key9 as i16)));
            Map::erase_key(m.as_pointer(), &__tmp_7.as_pointer())
        }) == 1_usize)
    );
    assert!(({ ((*r.upgrade().deref()).len() == 5_usize) }));
    let cit: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            ({
                let __tmp_8: Value<i16> = Rc::new(RefCell::new(7_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_8.as_pointer())
                    })
            }),
        ),
    ));
    let next: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        Map::erase(m.as_pointer(), &({ (*cit.borrow()).clone() })),
    ));
    assert!(
        (*next.borrow())
            == ({
                let __tmp_9: Value<i16> = Rc::new(RefCell::new(8_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_9.as_pointer())
                    })
            })
    );
    (*next.borrow_mut()) = Map::erase(m.as_pointer(), &({ (*next.borrow()).clone() }));
    assert!(
        (*next.borrow())
            == m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    assert!(({ ((*r.upgrade().deref()).len() == 3_usize) }));
    assert!(
        ({
            let __tmp_10: Value<i16> = Rc::new(RefCell::new(8_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_10.as_pointer()))
        }) == m
            .as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    let other_map: Value<
        Map<(Value<i32>, Value<i64>), (Value<(Value<i32>, Value<i64>)>, Value<f64>)>,
    > = Rc::new(RefCell::new(Map::new()));
    assert!(({ ((*other_map.borrow()).len() == 0_usize) }));
    let key0: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new((1 as i64))),
    )));
    let mut value: f64 = 2_f64;
    Map::subscript(other_map.as_pointer(), key0.as_pointer()).write(value);
    value = (Map::subscript(other_map.as_pointer(), key0.as_pointer()).read());
    assert!(({ ((*other_map.borrow()).len() == 1_usize) }));
    assert!(((Map::subscript(other_map.as_pointer(), key0.as_pointer()).read()) == value));
    assert!(({ ((*m.borrow()).len() == 3_usize) }));
    let mut k: i32 = 0;
    assert!(
        ({
            let __tmp_11: Value<i16> = Rc::new(RefCell::new((k as i16)));
            ((*m.borrow())
                .get(&__tmp_11.as_pointer())
                .expect("out of range!")
                .borrow()
                .1
                .as_pointer()
                .read())
        } == 5_u32)
    );
    k.prefix_inc();
    assert!(
        ({
            let __tmp_12: Value<i16> = Rc::new(RefCell::new((k as i16)));
            ((*m.borrow())
                .get(&__tmp_12.as_pointer())
                .expect("out of range!")
                .borrow()
                .1
                .as_pointer()
                .read())
        } == 4_u32)
    );
    k.prefix_inc();
    assert!(
        ({
            let __tmp_13: Value<i16> = Rc::new(RefCell::new((k as i16)));
            ((*m.borrow())
                .get(&__tmp_13.as_pointer())
                .expect("out of range!")
                .borrow()
                .1
                .as_pointer()
                .read())
        } == 4_u32)
    );
    let m2: Value<Map<i32, (Value<i32>, Value<bool>)>> = Rc::new(RefCell::new(Map::new()));
    assert!(({ ((*m2.borrow()).len() == 0_usize) }));
    let indexes: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: u32 = 60_u32;
    'loop_: while (i > 30_u32) {
        {
            let __a1 = (i as i32);
            (*indexes.borrow_mut()).push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 100_u32;
    'loop_: while (i > 60_u32) {
        {
            let __a1 = (i as i32);
            (*indexes.borrow_mut()).push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 30_u32;
    'loop_: while (i > 0_u32) {
        {
            let __a1 = (i as i32);
            (*indexes.borrow_mut()).push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*indexes.borrow()).len()) {
        let __rhs = ((i).wrapping_rem(2_u32) != 0);
        Map::subscript(m2.as_pointer(), indexes.decay().offset((i as usize))).write(__rhs);
        i.prefix_inc();
    }
    assert!(({ ((*m2.borrow()).len() == (*indexes.borrow()).len()) }));
    let mut last: i32 = -1_i32;
    'loop_: for pair in IterRange::new(
        m2.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<bool>)>| __m.begin()),
        m2.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<bool>)>| __m.end()),
    ) {
        let pair: Ptr<(Value<i32>, Value<bool>)> = pair.iter_deref();
        assert!(({ ({ (*(*pair.upgrade().deref()).0.borrow()) } > { last }) }));
        assert!(
            ({
                ({ ((*(*pair.upgrade().deref()).1.borrow()) as i32) } == {
                    ((*(*pair.upgrade().deref()).0.borrow()) % 2)
                })
            })
        );
        last = (*(*pair.upgrade().deref()).0.borrow());
    }
    let m3: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let inserted: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __init = (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2)));
        let __key = __init.0.as_pointer();
        let __inserted = m3
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.get(&__key).is_none());
        if __inserted {
            Map::subscript(m3.as_pointer().clone(), __key.clone()).write(__init.1.borrow().clone());
        }
        (
            Rc::new(RefCell::new(m3.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&__key),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ ((*(*inserted.borrow()).1.borrow()) as bool) }));
    assert!(
        ({
            ((*(*(*(*inserted.borrow()).0.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
                == 2)
        })
    );
    let not_inserted: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __init = (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(3)));
        let __key = __init.0.as_pointer();
        let __inserted = m3
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.get(&__key).is_none());
        if __inserted {
            Map::subscript(m3.as_pointer().clone(), __key.clone()).write(__init.1.borrow().clone());
        }
        (
            Rc::new(RefCell::new(m3.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&__key),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ !(*(*not_inserted.borrow()).1.borrow()) }));
    assert!(
        ({
            ((*(*(*(*not_inserted.borrow()).0.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
                == 2)
        })
    );
    {
        let __init = match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(4)),
            Rc::new(RefCell::new(5)),
        )))
        .as_pointer(),)
        {
            (__tmp_14,) => (
                Rc::new(RefCell::new(
                    ((*__tmp_14.upgrade().deref()).0.as_pointer().read()),
                )),
                Rc::new(RefCell::new(
                    ((*__tmp_14.upgrade().deref()).1.as_pointer().read()),
                )),
            ),
        };
        let __key = __init.0.as_pointer();
        let __inserted = m3
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.get(&__key).is_none());
        if __inserted {
            Map::subscript(m3.as_pointer().clone(), __key.clone()).write(__init.1.borrow().clone());
        }
        (
            Rc::new(RefCell::new(m3.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&__key),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    assert!(({ ((*m3.borrow()).len() == 2_usize) }));
    assert!(
        ((Map::subscript_move(
            m3.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 2)
    );
    assert!(
        ((Map::subscript_move(
            m3.as_pointer(),
            Rc::new(RefCell::new(4)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 5)
    );
    let hinted: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new({
        let _ = <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
            m3.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
        );
        let __init = (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(7)));
        let __key = __init.0.as_pointer();
        if m3
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.get(&__key).is_none())
        {
            Map::subscript(m3.as_pointer().clone(), __key.clone()).write(__init.1.borrow().clone());
        }
        m3.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&__key))
    }));
    assert!(
        ({
            ((*(*(*hinted.borrow()).iter_deref().upgrade().deref())
                .1
                .borrow())
                == 7)
        })
    );
    let key1: Value<i32> = Rc::new(RefCell::new(1));
    let tried: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __inserted = m3
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.get(&key1.as_pointer()).is_none());
        if __inserted {
            Map::subscript(m3.as_pointer().clone(), key1.as_pointer().clone()).write(9);
        }
        (
            Rc::new(RefCell::new(m3.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&key1.as_pointer()),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ !(*(*tried.borrow()).1.borrow()) }));
    assert!(
        ({
            ((*(*(*(*tried.borrow()).0.borrow())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
                == 2)
        })
    );
    let key8: Value<i32> = Rc::new(RefCell::new(8));
    let tried_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<i32>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __inserted = m3
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.get(&key8.as_pointer()).is_none());
        if __inserted {
            Map::subscript(m3.as_pointer().clone(), key8.as_pointer().clone()).write(9);
        }
        (
            Rc::new(RefCell::new(m3.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&key8.as_pointer()),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ ((*(*tried_new.borrow()).1.borrow()) as bool) }));
    assert!(
        ((Map::subscript_move(
            m3.as_pointer(),
            Rc::new(RefCell::new(8)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 9)
    );
    assert!(({ ((*m3.borrow()).len() == 4_usize) }));
    let points: Value<Map<i32, (Value<i32>, Value<Point>)>> = Rc::new(RefCell::new(Map::new()));
    let point_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Point>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let __inserted = points
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<Point>)>| {
                __m.get(&key1.as_pointer()).is_none()
            });
        if __inserted {
            Map::subscript(points.as_pointer().clone(), key1.as_pointer().clone())
                .write(Point::new_2({ 3 }, { 4 }));
        }
        (
            Rc::new(RefCell::new(points.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Point>)>| __m.find(&key1.as_pointer()),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ ((*(*point_new.borrow()).1.borrow()) as bool) }));
    assert!(
        (Map::subscript_move(
            points.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .with(|__s| __s.x)
            == 3)
    );
    assert!(
        (Map::subscript_move(
            points.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .with(|__s| __s.y)
            == 4)
    );
    {
        let __init = (
            Rc::new(RefCell::new(({ (*key8.borrow()) }))),
            Rc::new(RefCell::new(Point::new_2({ 5 }, { 6 }))),
        );
        let __key = __init.0.as_pointer();
        let __inserted = points
            .as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<Point>)>| __m.get(&__key).is_none());
        if __inserted {
            Map::subscript(points.as_pointer().clone(), __key.clone())
                .write(__init.1.borrow().clone());
        }
        (
            Rc::new(RefCell::new(points.as_pointer().with(
                |__m: &Map<i32, (Value<i32>, Value<Point>)>| __m.find(&__key),
            ))),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    assert!(
        (Map::subscript_move(
            points.as_pointer(),
            Rc::new(RefCell::new(8)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .with(|__s| __s.x)
            == 5)
    );
    assert!(
        (Map::subscript_move(
            points.as_pointer(),
            Rc::new(RefCell::new(8)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .with(|__s| __s.y)
            == 6)
    );
    assert!(({ ((*points.borrow()).len() == 2_usize) }));
    k = 0;
    let value_0: Ptr<u32> = ({
        let __tmp_15: Value<i16> = Rc::new(RefCell::new((k as i16)));
        (*m.borrow())
            .get(&__tmp_15.as_pointer())
            .expect("out of range!")
            .borrow()
            .1
            .as_pointer()
    });
    assert!(
        ({
            ((((((((*m.borrow()).len()).wrapping_add((x1 as usize)))
                .wrapping_add((x2 as usize)))
            .wrapping_add((x3 as usize)))
            .wrapping_add((x4 as usize)))
            .wrapping_add((x5 as usize)))
            .wrapping_add(((value_0.read()) as usize))
                == 21_usize)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
