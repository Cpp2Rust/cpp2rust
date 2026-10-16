extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: u32) {
    x = (x).wrapping_add(1_u32);
}
pub unsafe fn bar_1(x: *mut u32) {
    (*x) = (*x).wrapping_add(1_u32);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: BTreeMap<i16, Box<(i16, u32)>> = BTreeMap::new();
    (*&mut m
        .entry(0_i16.clone())
        .or_insert_with(|| Box::new((0_i16, <u32>::default())))
        .1) = 1_u32;
    (*&mut m
        .entry(1_i16.clone())
        .or_insert_with(|| Box::new((1_i16, <u32>::default())))
        .1) = 2_u32;
    (*&mut m
        .entry(2_i16.clone())
        .or_insert_with(|| Box::new((2_i16, <u32>::default())))
        .1) = 3_u32;
    assert!(((m.len()) == (3_usize)));
    assert!(
        ((*&mut m
            .entry(0_i16.clone())
            .or_insert_with(|| Box::new((0_i16, <u32>::default())))
            .1)
            == (1_u32))
    );
    assert!(
        ((*&mut m
            .entry(1_i16.clone())
            .or_insert_with(|| Box::new((1_i16, <u32>::default())))
            .1)
            == (2_u32))
    );
    assert!(
        ((*&mut m
            .entry(2_i16.clone())
            .or_insert_with(|| Box::new((2_i16, <u32>::default())))
            .1)
            == (3_u32))
    );
    let mut x: i32 = 4;
    (*&mut m
        .entry(1_i16.clone())
        .or_insert_with(|| Box::new((1_i16, <u32>::default())))
        .1) = (x as u32);
    assert!(((m.len()) == (3_usize)));
    assert!(
        ((*&mut m
            .entry(0_i16.clone())
            .or_insert_with(|| Box::new((0_i16, <u32>::default())))
            .1)
            == (1_u32))
    );
    assert!(
        ((*&mut m
            .entry(1_i16.clone())
            .or_insert_with(|| Box::new((1_i16, <u32>::default())))
            .1)
            == (4_u32))
    );
    assert!(
        ((*&mut m
            .entry(2_i16.clone())
            .or_insert_with(|| Box::new((2_i16, <u32>::default())))
            .1)
            == (3_u32))
    );
    (unsafe {
        foo_0(
            (*&mut m
                .entry(0_i16.clone())
                .or_insert_with(|| Box::new((0_i16, <u32>::default())))
                .1),
        )
    });
    assert!(
        ((*&mut m
            .entry(0_i16.clone())
            .or_insert_with(|| Box::new((0_i16, <u32>::default())))
            .1)
            == (1_u32))
    );
    (unsafe {
        bar_1(
            &mut (*&mut m
                .entry(2_i16.clone())
                .or_insert_with(|| Box::new((2_i16, <u32>::default())))
                .1),
        )
    });
    assert!(
        ((*&mut m
            .entry(2_i16.clone())
            .or_insert_with(|| Box::new((2_i16, <u32>::default())))
            .1)
            == (4_u32))
    );
    (*&mut m
        .entry(0_i16.clone())
        .or_insert_with(|| Box::new((0_i16, <u32>::default())))
        .1) = (*&mut m
        .entry(0_i16.clone())
        .or_insert_with(|| Box::new((0_i16, <u32>::default())))
        .1)
        .wrapping_add(
            (*&mut m
                .entry(2_i16.clone())
                .or_insert_with(|| Box::new((2_i16, <u32>::default())))
                .1),
        );
    assert!(
        ((*&mut m
            .entry(0_i16.clone())
            .or_insert_with(|| Box::new((0_i16, <u32>::default())))
            .1)
            == (5_u32))
    );
    let mut end: UnsafeBidirectionalIterator<(i16, u32)> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<(i16, u32)>>);
    let mut it: UnsafeBidirectionalIterator<(i16, u32)> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<(i16, u32)>>, &1_i16);
    let mut const_it: UnsafeBidirectionalIterator<(i16, u32)> =
        <UnsafeBidirectionalIterator<(i16, u32)>>::from(UnsafeMapIterator::find_key(
            &m as *const BTreeMap<i16, Box<(i16, u32)>>,
            &10_i16,
        ));
    let mut x1: u32 = if it == end {
        0_u32
    } else {
        (*it.iter_deref()).second
    };
    assert!(((x1) == (4_u32)));
    let mut x2: u32 = if const_it == <UnsafeBidirectionalIterator<(i16, u32)>>::from((end).clone())
    {
        0_u32
    } else {
        (*const_it.iter_deref()).second
    };
    assert!(((x2) == (0_u32)));
    let mut x3: u32 = if it == UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<(i16, u32)>>) {
        0_u32
    } else {
        (*it.iter_deref()).second
    };
    assert!(((x3) == (4_u32)));
    let mut x4: u32 = if const_it
        == <UnsafeBidirectionalIterator<(i16, u32)>>::from(UnsafeMapIterator::end(
            &m as *const BTreeMap<i16, Box<(i16, u32)>>,
        )) {
        0_u32
    } else {
        (*const_it.iter_deref()).second
    };
    assert!(((x4) == (0_u32)));
    (*&mut m
        .entry(4_i16.clone())
        .or_insert_with(|| Box::new((4_i16, <u32>::default())))
        .1) = 5_u32;
    let mut it4: UnsafeBidirectionalIterator<(i16, u32)> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<(i16, u32)>>, &4_i16);
    let mut p: *mut u32 = (&mut (*it4.iter_deref()).second as *mut u32);
    let mut x5: u32 = (*p);
    assert!(
        ((*&mut m
            .entry(4_i16.clone())
            .or_insert_with(|| Box::new((4_i16, <u32>::default())))
            .1)
            == (5_u32))
    );
    assert!((((*it4.iter_deref()).second) == (5_u32)));
    assert!(((*p) == (5_u32)));
    assert!(((x5) == (5_u32)));
    (*p).prefix_inc();
    assert!(
        ((*&mut m
            .entry(4_i16.clone())
            .or_insert_with(|| Box::new((4_i16, <u32>::default())))
            .1)
            == (6_u32))
    );
    assert!((((*it4.iter_deref()).second) == (6_u32)));
    assert!(((*p) == (6_u32)));
    assert!(((x5) == (5_u32)));
    let mut const_it4: UnsafeBidirectionalIterator<(i16, u32)> =
        <UnsafeBidirectionalIterator<(i16, u32)>>::from(UnsafeMapIterator::find_key(
            &m as *const BTreeMap<i16, Box<(i16, u32)>>,
            &4_i16,
        ));
    assert!((((*const_it4.iter_deref()).1) == (6_u32)));
    let r: *mut BTreeMap<i16, Box<(i16, u32)>> = &mut m;
    assert!((((*r).len()) == (4_usize)));
    assert!(
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<(i16, u32)>>, &4_i16)
            != UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<(i16, u32)>>)
    );
    UnsafeMapIterator::erase(&(*r) as *const BTreeMap<i16, Box<(i16, u32)>>, &it4.clone());
    assert!((((*r).len()) == (3_usize)));
    assert!(
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<(i16, u32)>>, &4_i16)
            == UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<(i16, u32)>>)
    );
    let mut other_map: BTreeMap<(i32, i64), Box<((i32, i64), f64)>> = BTreeMap::new();
    assert!(((other_map.len()) == (0_usize)));
    let mut key0: (i32, i64) = (1.into(), 1.into());
    let mut value: f64 = 2_f64;
    (*&mut other_map
        .entry(key0.clone())
        .or_insert_with(|| Box::new((key0, <f64>::default())))
        .1) = value;
    value = (*&mut other_map
        .entry(key0.clone())
        .or_insert_with(|| Box::new((key0, <f64>::default())))
        .1);
    assert!(((other_map.len()) == (1_usize)));
    assert!(
        ((*&mut other_map
            .entry(key0.clone())
            .or_insert_with(|| Box::new((key0, <f64>::default())))
            .1)
            == (value))
    );
    assert!(((m.len()) == (3_usize)));
    let mut k: i32 = 0;
    assert!(((*(&m.get(&(k as i16)).expect("out of range!").1 as *const u32)) == (5_u32)));
    k.prefix_inc();
    assert!(((*(&m.get(&(k as i16)).expect("out of range!").1 as *const u32)) == (4_u32)));
    k.prefix_inc();
    assert!(((*(&m.get(&(k as i16)).expect("out of range!").1 as *const u32)) == (4_u32)));
    let mut m2: BTreeMap<i32, Box<(i32, bool)>> = BTreeMap::new();
    assert!(((m2.len()) == (0_usize)));
    let mut indexes: Vec<i32> = Vec::new();
    let mut i: u32 = 60_u32;
    'loop_: while ((i) > (30_u32)) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 100_u32;
    'loop_: while ((i) > (60_u32)) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 30_u32;
    'loop_: while ((i) > (0_u32)) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (indexes.len())) {
        (*&mut m2
            .entry(indexes[(i as usize)].clone())
            .or_insert_with(|| Box::new((indexes[(i as usize)], <bool>::default())))
            .1) = ((i).wrapping_rem(2_u32) != 0);
        i.prefix_inc();
    }
    assert!(((m2.len()) == (indexes.len())));
    let mut last: i32 = -1_i32;
    'loop_: for __begin1 in UnsafeMapIterator::begin(&m2 as *const BTreeMap<i32, Box<(i32, bool)>>)
    {
        let pair: *mut (i32, bool) = &mut (*__begin1.iter_deref());
        assert!((((*pair).0) > (last)));
        assert!((((*pair).1 as i32) == (((*pair).0) % (2))));
        last = (*pair).0;
    }
    k = 0;
    let value_0: *const u32 = (&m.get(&(k as i16)).expect("out of range!").1 as *const u32);
    assert!(
        ((((((((m.len()).wrapping_add((x1 as usize))).wrapping_add((x2 as usize)))
            .wrapping_add((x3 as usize)))
        .wrapping_add((x4 as usize)))
        .wrapping_add((x5 as usize)))
        .wrapping_add(((*value_0) as usize)))
            == (21_usize))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
