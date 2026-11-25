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
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Map<i16, (Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(Map::new()));
    match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
        (__tmp_0,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_0, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(1_u32);
    match (Rc::new(RefCell::new(1_i16)).as_pointer(),) {
        (__tmp_1,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_1, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(2_u32);
    match (Rc::new(RefCell::new(2_i16)).as_pointer(),) {
        (__tmp_2,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_2, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(3_u32);
    assert!(({ ((*m.borrow()).len() == 3_usize) }));
    assert!(
        ((match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
            (__tmp_3,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_3, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 1_u32)
    );
    assert!(
        ((match (Rc::new(RefCell::new(1_i16)).as_pointer(),) {
            (__tmp_4,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_4, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 2_u32)
    );
    assert!(
        ((match (Rc::new(RefCell::new(2_i16)).as_pointer(),) {
            (__tmp_5,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_5, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 3_u32)
    );
    let mut x: i32 = 4;
    match (Rc::new(RefCell::new(1_i16)).as_pointer(),) {
        (__tmp_6,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_6, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write((x as u32));
    assert!(({ ((*m.borrow()).len() == 3_usize) }));
    assert!(
        ((match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
            (__tmp_7,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_7, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 1_u32)
    );
    assert!(
        ((match (Rc::new(RefCell::new(1_i16)).as_pointer(),) {
            (__tmp_8,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_8, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 4_u32)
    );
    assert!(
        ((match (Rc::new(RefCell::new(2_i16)).as_pointer(),) {
            (__tmp_9,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_9, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 3_u32)
    );
    ({
        foo_0(
            (match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
                (__tmp_10,) => {
                    ({ (*m.borrow_mut()).subscript_move(__tmp_10, |__k| cpp_move!(__k), || 0_u32) })
                }
            }
            .read()),
        )
    });
    assert!(
        ((match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
            (__tmp_11,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_11, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 1_u32)
    );
    ({
        bar_1(match (Rc::new(RefCell::new(2_i16)).as_pointer(),) {
            (__tmp_12,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_12, |__k| cpp_move!(__k), || 0_u32) })
            }
        })
    });
    assert!(
        ((match (Rc::new(RefCell::new(2_i16)).as_pointer(),) {
            (__tmp_13,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_13, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 4_u32)
    );
    let __rhs = (match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
        (__tmp_14,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_14, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .read())
    .wrapping_add(
        (match (Rc::new(RefCell::new(2_i16)).as_pointer(),) {
            (__tmp_15,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_15, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read()),
    );
    match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
        (__tmp_16,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_16, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(__rhs);
    assert!(
        ((match (Rc::new(RefCell::new(0_i16)).as_pointer(),) {
            (__tmp_17,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_17, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 5_u32)
    );
    let end: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end()),
    ));
    let it: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        ({
            let __tmp_18: Value<i16> = Rc::new(RefCell::new(1_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_18.as_pointer()))
        }),
    ));
    let const_it: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            ({
                let __tmp_19: Value<i16> = Rc::new(RefCell::new(10_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_19.as_pointer())
                    })
            }),
        ),
    ));
    let mut x1: u32 = if (*it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*({ (*it.borrow()).iter_deref() }).upgrade().deref())
            .1
            .borrow())
    };
    assert!((x1 == 4_u32));
    let mut x2: u32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(({ (*end.borrow()).clone() }))
    {
        0_u32
    } else {
        (*(*({ (*const_it.borrow()).iter_deref() }).upgrade().deref())
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
        (*(*({ (*it.borrow()).iter_deref() }).upgrade().deref())
            .1
            .borrow())
    };
    assert!((x3 == 4_u32));
    let mut x4: u32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end()),
        ) {
        0_u32
    } else {
        (*(*({ (*const_it.borrow()).iter_deref() }).upgrade().deref())
            .1
            .borrow())
    };
    assert!((x4 == 0_u32));
    match (Rc::new(RefCell::new(4_i16)).as_pointer(),) {
        (__tmp_20,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_20, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(5_u32);
    let it4: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        ({
            let __tmp_21: Value<i16> = Rc::new(RefCell::new(4_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_21.as_pointer()))
        }),
    ));
    let mut p: Ptr<u32> = ((*({ (*it4.borrow()).iter_deref() }).upgrade().deref())
        .1
        .as_pointer());
    let mut x5: u32 = (p.read());
    assert!(
        ((match (Rc::new(RefCell::new(4_i16)).as_pointer(),) {
            (__tmp_22,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_22, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 5_u32)
    );
    assert!(
        ({
            ((*(*({ (*it4.borrow()).iter_deref() }).upgrade().deref())
                .1
                .borrow())
                == 5_u32)
        })
    );
    assert!(((p.read()) == 5_u32));
    assert!((x5 == 5_u32));
    p.with_mut(|__v| {
        __v.prefix_inc();
    });
    assert!(
        ((match (Rc::new(RefCell::new(4_i16)).as_pointer(),) {
            (__tmp_23,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_23, |__k| cpp_move!(__k), || 0_u32) })
            }
        }
        .read())
            == 6_u32)
    );
    assert!(
        ({
            ((*(*({ (*it4.borrow()).iter_deref() }).upgrade().deref())
                .1
                .borrow())
                == 6_u32)
        })
    );
    assert!(((p.read()) == 6_u32));
    assert!((x5 == 5_u32));
    let const_it4: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            ({
                let __tmp_24: Value<i16> = Rc::new(RefCell::new(4_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_24.as_pointer())
                    })
            }),
        ),
    ));
    assert!(
        ({
            ((*(*({ (*const_it4.borrow()).iter_deref() }).upgrade().deref())
                .1
                .borrow())
                == 6_u32)
        })
    );
    let r: Ptr<Map<i16, (Value<i16>, Value<u32>)>> = m.as_pointer();
    assert!(({ ((*r.upgrade().deref()).len() == 4_usize) }));
    assert!(
        ({
            let __tmp_25: Value<i16> = Rc::new(RefCell::new(4_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_25.as_pointer()))
        }) != m
            .as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    Map::erase((r).clone(), &({ (*it4.borrow()).clone() }));
    assert!(({ ((*r.upgrade().deref()).len() == 3_usize) }));
    assert!(
        ({
            let __tmp_26: Value<i16> = Rc::new(RefCell::new(4_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_26.as_pointer()))
        }) == m
            .as_pointer()
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    match (Rc::new(RefCell::new(7_i16)).as_pointer(),) {
        (__tmp_27,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_27, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(7_u32);
    match (Rc::new(RefCell::new(8_i16)).as_pointer(),) {
        (__tmp_28,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_28, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(8_u32);
    match (Rc::new(RefCell::new(9_i16)).as_pointer(),) {
        (__tmp_29,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_29, |__k| cpp_move!(__k), || 0_u32) })
        }
    }
    .write(9_u32);
    assert!(({ ((*r.upgrade().deref()).len() == 6_usize) }));
    assert!(
        (({
            let __tmp_30: Value<i16> = Rc::new(RefCell::new(10_i16));
            Map::erase_key(m.as_pointer(), &__tmp_30.as_pointer())
        }) == 0_usize)
    );
    let mut key9: i32 = 9;
    assert!(
        (({
            let __tmp_31: Value<i16> = Rc::new(RefCell::new((key9 as i16)));
            Map::erase_key(m.as_pointer(), &__tmp_31.as_pointer())
        }) == 1_usize)
    );
    assert!(({ ((*r.upgrade().deref()).len() == 5_usize) }));
    let cit: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            ({
                let __tmp_32: Value<i16> = Rc::new(RefCell::new(7_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_32.as_pointer())
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
                let __tmp_33: Value<i16> = Rc::new(RefCell::new(8_i16));
                m.as_pointer()
                    .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| {
                        __m.find(&__tmp_33.as_pointer())
                    })
            })
    );
    {
        (*next.borrow_mut()) = Map::erase(m.as_pointer(), &({ (*next.borrow()).clone() }));
    };
    assert!(
        (*next.borrow())
            == m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    assert!(({ ((*r.upgrade().deref()).len() == 3_usize) }));
    assert!(
        ({
            let __tmp_34: Value<i16> = Rc::new(RefCell::new(8_i16));
            m.as_pointer()
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&__tmp_34.as_pointer()))
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
    match (key0.as_pointer(),) {
        (__tmp_35,) => ({ (*other_map.borrow_mut()).subscript(__tmp_35, || 0_f64) }),
    }
    .write(value);
    value = (match (key0.as_pointer(),) {
        (__tmp_36,) => ({ (*other_map.borrow_mut()).subscript(__tmp_36, || 0_f64) }),
    }
    .read());
    assert!(({ ((*other_map.borrow()).len() == 1_usize) }));
    assert!(
        ((match (key0.as_pointer(),) {
            (__tmp_37,) => {
                ({ (*other_map.borrow_mut()).subscript(__tmp_37, || 0_f64) })
            }
        }
        .read())
            == value)
    );
    assert!(({ ((*m.borrow()).len() == 3_usize) }));
    let mut k: i32 = 0;
    assert!(
        ({
            let __tmp_38: Value<i16> = Rc::new(RefCell::new((k as i16)));
            ((*m.borrow())
                .get(&__tmp_38.as_pointer())
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
            let __tmp_39: Value<i16> = Rc::new(RefCell::new((k as i16)));
            ((*m.borrow())
                .get(&__tmp_39.as_pointer())
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
            let __tmp_40: Value<i16> = Rc::new(RefCell::new((k as i16)));
            ((*m.borrow())
                .get(&__tmp_40.as_pointer())
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
    {
        let mut i: u32 = 60_u32;
        'loop_: while (i > 30_u32) {
            'for_body: {
                {
                    let __a1 = (i as i32);
                    (*indexes.borrow_mut()).push(__a1)
                };
            }
            i.prefix_dec();
        }
    }
    {
        let mut i: u32 = 100_u32;
        'loop_: while (i > 60_u32) {
            'for_body: {
                {
                    let __a1 = (i as i32);
                    (*indexes.borrow_mut()).push(__a1)
                };
            }
            i.prefix_dec();
        }
    }
    {
        let mut i: u32 = 30_u32;
        'loop_: while (i > 0_u32) {
            'for_body: {
                {
                    let __a1 = (i as i32);
                    (*indexes.borrow_mut()).push(__a1)
                };
            }
            i.prefix_dec();
        }
    }
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < (*indexes.borrow()).len()) {
            'for_body: {
                let __rhs = ((i).wrapping_rem(2_u32) != 0);
                match (indexes.decay().offset((i as usize)),) {
                    (__tmp_41,) => ({ (*m2.borrow_mut()).subscript(__tmp_41, || false) }),
                }
                .write(__rhs);
            }
            i.prefix_inc();
        }
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
        let mut __tmp_42 = { (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))) };
        let (__it, __inserted) =
            Map::emplace(m3.as_pointer(), &__tmp_42.0.as_pointer(), || __tmp_42);
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ ((*(*inserted.borrow()).1.borrow()) as bool) }));
    assert!(
        ({
            ((*(*({ (*(*inserted.borrow()).0.borrow()).iter_deref() })
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
        let mut __tmp_43 = { (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(3))) };
        let (__it, __inserted) =
            Map::emplace(m3.as_pointer(), &__tmp_43.0.as_pointer(), || __tmp_43);
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ !(*(*not_inserted.borrow()).1.borrow()) }));
    assert!(
        ({
            ((*(*({ (*(*not_inserted.borrow()).0.borrow()).iter_deref() })
                .upgrade()
                .deref())
            .1
            .borrow())
                == 2)
        })
    );
    {
        let mut __tmp_45 = {
            match (Rc::new(RefCell::new((
                Rc::new(RefCell::new(4)),
                Rc::new(RefCell::new(5)),
            )))
            .as_pointer(),)
            {
                (__tmp_44,) => (
                    Rc::new(RefCell::new(
                        ((*__tmp_44.upgrade().deref()).0.as_pointer().read()),
                    )),
                    Rc::new(RefCell::new(
                        ((*__tmp_44.upgrade().deref()).1.as_pointer().read()),
                    )),
                ),
            }
        };
        let (__it, __inserted) =
            Map::emplace(m3.as_pointer(), &__tmp_45.0.as_pointer(), || __tmp_45);
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    assert!(({ ((*m3.borrow()).len() == 2_usize) }));
    assert!(
        ((match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_46,) => {
                ({ (*m3.borrow_mut()).subscript_move(__tmp_46, |__k| cpp_move!(__k), || 0_i32) })
            }
        }
        .read())
            == 2)
    );
    assert!(
        ((match (Rc::new(RefCell::new(4)).as_pointer(),) {
            (__tmp_47,) => {
                ({ (*m3.borrow_mut()).subscript_move(__tmp_47, |__k| cpp_move!(__k), || 0_i32) })
            }
        }
        .read())
            == 5)
    );
    let hinted: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new({
        let mut __tmp_48 = {
            <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from(
                m3.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
            )
        };
        let mut __tmp_49 = { (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(7))) };
        let _ = (__tmp_48).clone();
        Map::emplace(m3.as_pointer(), &__tmp_49.0.as_pointer(), || __tmp_49).0
    }));
    assert!(
        ({
            ((*(*({ (*hinted.borrow()).iter_deref() }).upgrade().deref())
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
        let (__it, __inserted) = Map::emplace(m3.as_pointer(), &key1.as_pointer(), || {
            (
                Rc::new(RefCell::new(key1.as_pointer().read())),
                Rc::new(RefCell::new(9)),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ !(*(*tried.borrow()).1.borrow()) }));
    assert!(
        ({
            ((*(*({ (*(*tried.borrow()).0.borrow()).iter_deref() })
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
        let (__it, __inserted) = Map::emplace(m3.as_pointer(), &key8.as_pointer(), || {
            (
                Rc::new(RefCell::new(key8.as_pointer().read())),
                Rc::new(RefCell::new(9)),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ ((*(*tried_new.borrow()).1.borrow()) as bool) }));
    assert!(
        ((match (Rc::new(RefCell::new(8)).as_pointer(),) {
            (__tmp_50,) => {
                ({ (*m3.borrow_mut()).subscript_move(__tmp_50, |__k| cpp_move!(__k), || 0_i32) })
            }
        }
        .read())
            == 9)
    );
    assert!(({ ((*m3.borrow()).len() == 4_usize) }));
    let points: Value<Map<i32, (Value<i32>, Value<Point>)>> = Rc::new(RefCell::new(Map::new()));
    let point_new: Value<(
        Value<BidirectionalIterator<(Value<i32>, Value<Point>)>>,
        Value<bool>,
    )> = Rc::new(RefCell::new({
        let (__it, __inserted) = Map::emplace(points.as_pointer(), &key1.as_pointer(), || {
            (
                Rc::new(RefCell::new(key1.as_pointer().read())),
                Rc::new(RefCell::new(Point::new_2({ 3 }, { 4 }))),
            )
        });
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    }));
    assert!(({ ((*(*point_new.borrow()).1.borrow()) as bool) }));
    assert!(
        (match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_51,) => {
                ({
                    (*points.borrow_mut()).subscript_move(
                        __tmp_51,
                        |__k| cpp_move!(__k),
                        || <Point>::default(),
                    )
                })
            }
        }
        .with(|__s| __s.x)
            == 3)
    );
    assert!(
        (match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_52,) => {
                ({
                    (*points.borrow_mut()).subscript_move(
                        __tmp_52,
                        |__k| cpp_move!(__k),
                        || <Point>::default(),
                    )
                })
            }
        }
        .with(|__s| __s.y)
            == 4)
    );
    {
        let mut __tmp_53 = {
            (
                Rc::new(RefCell::new(({ (*key8.borrow()) }))),
                Rc::new(RefCell::new(Point::new_2({ 5 }, { 6 }))),
            )
        };
        let (__it, __inserted) =
            Map::emplace(points.as_pointer(), &__tmp_53.0.as_pointer(), || __tmp_53);
        (
            Rc::new(RefCell::new(__it)),
            Rc::new(RefCell::new(__inserted)),
        )
    };
    assert!(
        (match (Rc::new(RefCell::new(8)).as_pointer(),) {
            (__tmp_54,) => {
                ({
                    (*points.borrow_mut()).subscript_move(
                        __tmp_54,
                        |__k| cpp_move!(__k),
                        || <Point>::default(),
                    )
                })
            }
        }
        .with(|__s| __s.x)
            == 5)
    );
    assert!(
        (match (Rc::new(RefCell::new(8)).as_pointer(),) {
            (__tmp_55,) => {
                ({
                    (*points.borrow_mut()).subscript_move(
                        __tmp_55,
                        |__k| cpp_move!(__k),
                        || <Point>::default(),
                    )
                })
            }
        }
        .with(|__s| __s.y)
            == 6)
    );
    assert!(({ ((*points.borrow()).len() == 2_usize) }));
    let mi: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        m3.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
    ));
    let mr: Ptr<BidirectionalIterator<(Value<i32>, Value<i32>)>> = {
        (*mi.borrow_mut()).iter_inc();
        mi.as_pointer()
    };
    assert!(
        ({
            ({ (mr).clone() } == { (mi.as_pointer()) })
                && ((*(*({ (*mi.borrow()).iter_deref() }).upgrade().deref())
                    .0
                    .borrow())
                    == 4)
        })
    );
    let md: Ptr<BidirectionalIterator<(Value<i32>, Value<i32>)>> = {
        (*mi.borrow_mut()).iter_dec();
        mi.as_pointer()
    };
    assert!(
        ({
            ({ (md).clone() } == { (mi.as_pointer()) })
                && ((*(*({ (*mi.borrow()).iter_deref() }).upgrade().deref())
                    .0
                    .borrow())
                    == 1)
        })
    );
    k = 0;
    let value_0: Ptr<u32> = ({
        let __tmp_56: Value<i16> = Rc::new(RefCell::new((k as i16)));
        (*m.borrow())
            .get(&__tmp_56.as_pointer())
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
    let pk1: Value<i32> = Rc::new(RefCell::new(0));
    let pk2: Value<i32> = Rc::new(RefCell::new(0));
    let ptr_counts: Value<Map<Ptr<i32>, (Value<Ptr<i32>>, Value<i32>)>> =
        Rc::new(RefCell::new(Map::new()));
    let ptr_key: Value<Ptr<i32>> = Rc::new(RefCell::new((pk1.as_pointer())));
    ({ (*ptr_counts.borrow_mut()).subscript(ptr_key.as_pointer(), || 0_i32) }).write(1);
    (*ptr_key.borrow_mut()) = (pk2.as_pointer());
    ({ (*ptr_counts.borrow_mut()).subscript(ptr_key.as_pointer(), || 0_i32) }).write(2);
    {
        ({ (*ptr_counts.borrow_mut()).subscript(ptr_key.as_pointer(), || 0_i32) })
            .with_mut(|__v| *__v = *__v + 3)
    };
    assert!(({ ((*ptr_counts.borrow()).len() == 2_usize) }));
    assert!(
        ((({ (*ptr_counts.borrow_mut()).subscript(ptr_key.as_pointer(), || 0_i32) }).read()) == 5)
    );
    (*ptr_key.borrow_mut()) = (pk1.as_pointer());
    assert!(
        ((({ (*ptr_counts.borrow_mut()).subscript(ptr_key.as_pointer(), || 0_i32) }).read()) == 1)
    );
    {
        match (Rc::new(RefCell::new((pk2.as_pointer()))).as_pointer(),) {
            (__tmp_57,) => {
                ({
                    (*ptr_counts.borrow_mut()).subscript_move(
                        __tmp_57,
                        |__k| cpp_move!(__k),
                        || 0_i32,
                    )
                })
            }
        }
        .with_mut(|__v| *__v = *__v + 1)
    };
    assert!(
        ((match (Rc::new(RefCell::new((pk2.as_pointer()))).as_pointer(),) {
            (__tmp_58,) => {
                ({
                    (*ptr_counts.borrow_mut()).subscript_move(
                        __tmp_58,
                        |__k| cpp_move!(__k),
                        || 0_i32,
                    )
                })
            }
        }
        .read())
            == 6)
    );
    assert!(({ ((*pk1.borrow()) == 0) && ((*pk2.borrow()) == 0) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
