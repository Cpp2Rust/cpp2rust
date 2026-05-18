extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: u32) {
    let x: Value<u32> = Rc::new(RefCell::new(x));
    let __rhs = (*x.borrow()).wrapping_add(1_u32);
    (*x.borrow_mut()) = __rhs;
}
pub fn bar_1(x: Ptr<u32>) {
    let __rhs = (x.read()).wrapping_add(1_u32);
    x.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<BTreeMap<i16, Value<u32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(0_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(1_u32);
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(1_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(2_u32);
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(2_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(3_u32);
    assert!(((*m.borrow()).len() as u64 == 3_u64));
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 1_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(1_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 2_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 3_u32)
    );
    let x: Value<i32> = Rc::new(RefCell::new(4));
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(1_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(((*x.borrow()) as u32));
    assert!(((*m.borrow()).len() as u64 == 3_u64));
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 1_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(1_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 4_u32)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 3_u32)
    );
    ({
        let _x: u32 = ((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read());
        foo_0(_x)
    });
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 1_u32)
    );
    ({
        let _x: Ptr<u32> = (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>).with_mut(
            |__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            },
        );
        bar_1(_x)
    });
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 4_u32)
    );
    let __rhs = ((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(0_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .read())
    .wrapping_add(
        ((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(2_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read()),
    );
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(0_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(__rhs);
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(0_i16.clone())
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
    let x1: Value<u32> = Rc::new(RefCell::new(if (*it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*it.borrow()).second().borrow())
    }));
    assert!(((*x1.borrow()) == 4_u32));
    let x2: Value<u32> = Rc::new(RefCell::new(if (*const_it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*const_it.borrow()).second().borrow())
    }));
    assert!(((*x2.borrow()) == 0_u32));
    let x3: Value<u32> = Rc::new(RefCell::new(
        if (*it.borrow())
            == RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
        {
            0_u32
        } else {
            (*(*it.borrow()).second().borrow())
        },
    ));
    assert!(((*x3.borrow()) == 4_u32));
    let x4: Value<u32> = Rc::new(RefCell::new(
        if (*const_it.borrow())
            == RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
        {
            0_u32
        } else {
            (*(*const_it.borrow()).second().borrow())
        },
    ));
    assert!(((*x4.borrow()) == 0_u32));
    (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
        .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
            __v.entry(4_i16.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                .as_pointer()
        })
        .write(5_u32);
    let it4: Value<RefcountMapIter<i16, u32>> = Rc::new(RefCell::new(RefcountMapIter::find_key(
        (m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>),
        &4_i16,
    )));
    let p: Value<Ptr<u32>> = Rc::new(RefCell::new(((*it4.borrow()).second().as_pointer())));
    let x5: Value<u32> = Rc::new(RefCell::new(((*p.borrow()).read())));
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(4_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 5_u32)
    );
    assert!(((*(*it4.borrow()).second().borrow()) == 5_u32));
    assert!((((*p.borrow()).read()) == 5_u32));
    assert!(((*x5.borrow()) == 5_u32));
    (*p.borrow()).with_mut(|__v| __v.prefix_inc());
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>)
            .with_mut(|__v: &mut BTreeMap<i16, Value<u32>>| {
                __v.entry(4_i16.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<u32>::default())))
                    .as_pointer()
            })
            .read())
            == 6_u32)
    );
    assert!(((*(*it4.borrow()).second().borrow()) == 6_u32));
    assert!((((*p.borrow()).read()) == 6_u32));
    assert!(((*x5.borrow()) == 5_u32));
    let r: Ptr<BTreeMap<i16, Value<u32>>> = m.as_pointer();
    assert!(((*r.upgrade().deref()).len() as u64 == 4_u64));
    assert!(
        RefcountMapIter::find_key((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>), &4_i16)
            != RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
    );
    RefcountMapIter::erase(
        ((r).clone() as Ptr<BTreeMap<i16, Value<u32>>>),
        &(*it4.borrow()),
    );
    assert!(((*r.upgrade().deref()).len() as u64 == 3_u64));
    assert!(
        RefcountMapIter::find_key((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>), &4_i16)
            == RefcountMapIter::end((m.as_pointer() as Ptr<BTreeMap<i16, Value<u32>>>))
    );
    let other_map: Value<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    assert!(((*other_map.borrow()).len() as u64 == 0_u64));
    let key0: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
    )));
    let value: Value<f64> = Rc::new(RefCell::new(2_f64));
    (other_map.as_pointer() as Ptr<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<(Value<i32>, Value<i64>), Value<f64>>| {
            __v.entry((*key0.borrow()).clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write((*value.borrow()));
    (*value.borrow_mut()) = ((other_map.as_pointer()
        as Ptr<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<(Value<i32>, Value<i64>), Value<f64>>| {
            __v.entry((*key0.borrow()).clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .read());
    assert!(((*other_map.borrow()).len() as u64 == 1_u64));
    assert!(
        (((other_map.as_pointer() as Ptr<BTreeMap<(Value<i32>, Value<i64>), Value<f64>>>)
            .with_mut(|__v: &mut BTreeMap<(Value<i32>, Value<i64>), Value<f64>>| {
                __v.entry((*key0.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                    .as_pointer()
            })
            .read())
            == (*value.borrow()))
    );
    assert!(((*m.borrow()).len() as u64 == 3_u64));
    let k: Value<i32> = Rc::new(RefCell::new(0));
    assert!(
        (((*m.borrow())
            .get(&((*k.borrow()) as i16))
            .expect("out of range!")
            .as_pointer()
            .read())
            == 5_u32)
    );
    (*k.borrow_mut()).prefix_inc();
    assert!(
        (((*m.borrow())
            .get(&((*k.borrow()) as i16))
            .expect("out of range!")
            .as_pointer()
            .read())
            == 4_u32)
    );
    (*k.borrow_mut()).prefix_inc();
    assert!(
        (((*m.borrow())
            .get(&((*k.borrow()) as i16))
            .expect("out of range!")
            .as_pointer()
            .read())
            == 4_u32)
    );
    let m2: Value<BTreeMap<i32, Value<bool>>> = Rc::new(RefCell::new(BTreeMap::new()));
    assert!(((*m2.borrow()).len() as u64 == 0_u64));
    let indexes: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<u32> = Rc::new(RefCell::new(60_u32));
    'loop_: while ((*i.borrow()) > 30_u32) {
        (*indexes.borrow_mut()).push(((*i.borrow()) as i32));
        (*i.borrow_mut()).prefix_dec();
    }
    let i: Value<u32> = Rc::new(RefCell::new(100_u32));
    'loop_: while ((*i.borrow()) > 60_u32) {
        (*indexes.borrow_mut()).push(((*i.borrow()) as i32));
        (*i.borrow_mut()).prefix_dec();
    }
    let i: Value<u32> = Rc::new(RefCell::new(30_u32));
    'loop_: while ((*i.borrow()) > 0_u32) {
        (*indexes.borrow_mut()).push(((*i.borrow()) as i32));
        (*i.borrow_mut()).prefix_dec();
    }
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*indexes.borrow()).len() as u64) {
        let __rhs = ((*i.borrow()).wrapping_rem(2_u32) != 0);
        (m2.as_pointer() as Ptr<BTreeMap<i32, Value<bool>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<bool>>| {
                __v.entry(
                    ((indexes.as_pointer() as Ptr<i32>)
                        .offset(((*i.borrow()) as u64) as isize)
                        .read())
                    .clone(),
                )
                .or_insert_with(|| Rc::new(RefCell::new(<bool>::default())))
                .as_pointer()
            })
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(((*m2.borrow()).len() as u64 == (*indexes.borrow()).len() as u64));
    let last: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'loop_: for pair in RefcountMapIter::begin(m2.as_pointer()) {
        assert!({
            let _lhs = (*pair.first().borrow());
            _lhs > (*last.borrow())
        });
        assert!({
            let _lhs = ((*pair.second().borrow()) as i32);
            _lhs == ((*pair.first().borrow()) % 2)
        });
        (*last.borrow_mut()) = (*pair.first().borrow());
    }
    (*k.borrow_mut()) = 0;
    let value_0: Ptr<u32> = (*m.borrow())
        .get(&((*k.borrow()) as i16))
        .expect("out of range!")
        .as_pointer();
    return (((((((((*m.borrow()).len() as u64).wrapping_add(((*x1.borrow()) as u64)))
        .wrapping_add(((*x2.borrow()) as u64)))
    .wrapping_add(((*x3.borrow()) as u64)))
    .wrapping_add(((*x4.borrow()) as u64)))
    .wrapping_add(((*x5.borrow()) as u64)))
    .wrapping_add(((value_0.read()) as u64))) as i32);
}
