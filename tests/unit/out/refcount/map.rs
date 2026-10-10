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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<BTreeMap<i16, Value<u32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(0_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(1_u32);
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(1_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(2_u32);
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(2_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(3_u32);
    assert!(((*m.borrow()).len() == 3_usize));
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 1_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(1_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 2_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 3_u32)
    );
    let mut x: i32 = 4;
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(1_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write((x as u32));
    assert!(((*m.borrow()).len() == 3_usize));
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 1_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(1_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 4_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 3_u32)
    );
    ({
        foo_0(
            ((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
                .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                    __v.entry(0_i16)
                        .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                        .as_pointer()
                })
                .read()),
        )
    });
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 1_u32)
    );
    ({
        bar_1((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>).with_mut(
            |__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            },
        ))
    });
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 4_u32)
    );
    let __rhs = ((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(0_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .read())
    .wrapping_add(
        ((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read()),
    );
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(0_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(__rhs);
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 5_u32)
    );
    let end: Value<RefcountMapIter<i16, u32>> = Rc::new(RefCell::new(RefcountMapIter::end(
        (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>),
    )));
    let it: Value<RefcountMapIter<i16, u32>> = Rc::new(RefCell::new(RefcountMapIter::find_key(
        (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>),
        &1_i16,
    )));
    let const_it: Value<RefcountMapIter<i16, u32>> = Rc::new(RefCell::new(
        RefcountMapIter::find_key((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>), &10_i16),
    ));
    let mut x1: u32 = if (*it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*it.borrow()).second().borrow())
    };
    assert!((x1 == 4_u32));
    let mut x2: u32 = if (*const_it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*const_it.borrow()).second().borrow())
    };
    assert!((x2 == 0_u32));
    let mut x3: u32 = if (*it.borrow())
        == RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
    {
        0_u32
    } else {
        (*(*it.borrow()).second().borrow())
    };
    assert!((x3 == 4_u32));
    let mut x4: u32 = if (*const_it.borrow())
        == RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
    {
        0_u32
    } else {
        (*(*const_it.borrow()).second().borrow())
    };
    assert!((x4 == 0_u32));
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(4_i16)
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(5_u32);
    let it4: Value<RefcountMapIter<i16, u32>> = Rc::new(RefCell::new(RefcountMapIter::find_key(
        (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>),
        &4_i16,
    )));
    let mut p: Ptr<u32> = ((*it4.borrow()).second().as_pointer());
    let mut x5: u32 = (p.read());
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(4_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 5_u32)
    );
    assert!(((*(*it4.borrow()).second().borrow()) == 5_u32));
    assert!(((p.read()) == 5_u32));
    assert!((x5 == 5_u32));
    p.with_mut(|__v| __v.prefix_inc());
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(4_i16)
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 6_u32)
    );
    assert!(((*(*it4.borrow()).second().borrow()) == 6_u32));
    assert!(((p.read()) == 6_u32));
    assert!((x5 == 5_u32));
    let r: Ptr<BTreeMap<i16, Value<u32>>> = m.as_pointer();
    assert!(((*r.upgrade().deref()).len() == 4_usize));
    assert!(
        RefcountMapIter::find_key((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>), &4_i16)
            != RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
    );
    RefcountMapIter::erase(
        ((r).clone() as Ptr<BTreeMap<i16, Value<u32>>>),
        &(*it4.borrow()).clone(),
    );
    assert!(((*r.upgrade().deref()).len() == 3_usize));
    assert!(
        RefcountMapIter::find_key((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>), &4_i16)
            == RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
    );
    let other_map: Value<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    assert!(((*other_map.borrow()).len() == 0_usize));
    let key0: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
    )));
    let mut value: f64 = 2_f64;
    (other_map.as_pointer() as Ptr<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<(Value<i32>, Value<i64>), Value<f64>>| {
            __v.entry((*key0.borrow()).clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(value);
    value = ((other_map.as_pointer() as Ptr<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<(Value<i32>, Value<i64>), Value<f64>>| {
            __v.entry((*key0.borrow()).clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .read());
    assert!(((*other_map.borrow()).len() == 1_usize));
    assert!(
        (((other_map.as_pointer() as Ptr<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>>)
            .with_mut(|__v: &mut BTreeMap<(Value<i32>, Value<i64>), Value<f64>>| {
                __v.entry((*key0.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                    .as_pointer()
            })
            .read())
            == value)
    );
    assert!(((*m.borrow()).len() == 3_usize));
    let mut k: i32 = 0;
    assert!(
        (((*m.borrow())
            .get(&(k as i16))
            .expect("out of range!")
            .as_pointer()
            .read())
            == 5_u32)
    );
    k.prefix_inc();
    assert!(
        (((*m.borrow())
            .get(&(k as i16))
            .expect("out of range!")
            .as_pointer()
            .read())
            == 4_u32)
    );
    k.prefix_inc();
    assert!(
        (((*m.borrow())
            .get(&(k as i16))
            .expect("out of range!")
            .as_pointer()
            .read())
            == 4_u32)
    );
    let m2: Value<BTreeMap<i32, Value<bool>>> = Rc::new(RefCell::new(BTreeMap::new()));
    assert!(((*m2.borrow()).len() == 0_usize));
    let mut indexes: Vec<i32> = Vec::new();
    let mut i: u32 = 60_u32;
    'loop_: while (i > 30_u32) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 100_u32;
    'loop_: while (i > 60_u32) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 30_u32;
    'loop_: while (i > 0_u32) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < indexes.len()) {
        let __rhs = ((i).wrapping_rem(2_u32) != 0);
        (m2.as_pointer() as Ptr<BTreeMap<i32, Value<bool>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<bool>>| {
                __v.entry(indexes[(i as usize)])
                    .or_insert_with(|| Rc::new(RefCell::new(<bool>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        i.prefix_inc();
    }
    assert!(((*m2.borrow()).len() == indexes.len()));
    let mut last: i32 = -1_i32;
    'loop_: for pair in RefcountMapIter::begin(m2.as_pointer()) {
        assert!(({ (*pair.first().borrow()) } > { last }));
        assert!(({ ((*pair.second().borrow()) as i32) } == { ((*pair.first().borrow()) % 2) }));
        last = (*pair.first().borrow());
    }
    k = 0;
    let value_0: Ptr<u32> = (*m.borrow())
        .get(&(k as i16))
        .expect("out of range!")
        .as_pointer();
    assert!(
        ((((((((*m.borrow()).len()).wrapping_add((x1 as usize))).wrapping_add((x2 as usize)))
            .wrapping_add((x3 as usize)))
        .wrapping_add((x4 as usize)))
        .wrapping_add((x5 as usize)))
        .wrapping_add(((value_0.read()) as usize))
            == 21_usize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
