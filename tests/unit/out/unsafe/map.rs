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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: BTreeMap<i16, Box<u32>> = BTreeMap::new();
    (*m.entry(0_i16).or_default().as_mut()) = 1_u32;
    (*m.entry(1_i16).or_default().as_mut()) = 2_u32;
    (*m.entry(2_i16).or_default().as_mut()) = 3_u32;
    assert!(((m.len() as u64) == (3_u64)));
    assert!(((*m.entry(0_i16).or_default().as_mut()) == (1_u32)));
    assert!(((*m.entry(1_i16).or_default().as_mut()) == (2_u32)));
    assert!(((*m.entry(2_i16).or_default().as_mut()) == (3_u32)));
    let mut x: i32 = 4;
    (*m.entry(1_i16).or_default().as_mut()) = (x as u32);
    assert!(((m.len() as u64) == (3_u64)));
    assert!(((*m.entry(0_i16).or_default().as_mut()) == (1_u32)));
    assert!(((*m.entry(1_i16).or_default().as_mut()) == (4_u32)));
    assert!(((*m.entry(2_i16).or_default().as_mut()) == (3_u32)));
    (unsafe {
        let _x: u32 = (*m.entry(0_i16).or_default().as_mut());
        foo_0(_x)
    });
    assert!(((*m.entry(0_i16).or_default().as_mut()) == (1_u32)));
    (unsafe {
        let _x: *mut u32 = &mut (*m.entry(2_i16).or_default().as_mut()) as *mut u32;
        bar_1(_x)
    });
    assert!(((*m.entry(2_i16).or_default().as_mut()) == (4_u32)));
    (*m.entry(0_i16).or_default().as_mut()) = (*m.entry(0_i16).or_default().as_mut())
        .wrapping_add((*m.entry(2_i16).or_default().as_mut()));
    assert!(((*m.entry(0_i16).or_default().as_mut()) == (5_u32)));
    let mut end: UnsafeMapIterator<i16, u32> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<u32>>);
    let mut it: UnsafeMapIterator<i16, u32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<u32>>, &1_i16);
    let mut const_it: UnsafeMapIterator<i16, u32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<u32>>, &10_i16);
    let mut x1: u32 = if it == end { 0_u32 } else { *it.second() };
    assert!(((x1) == (4_u32)));
    let mut x2: u32 = if const_it == end {
        0_u32
    } else {
        *const_it.second()
    };
    assert!(((x2) == (0_u32)));
    let mut x3: u32 = if it == UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<u32>>) {
        0_u32
    } else {
        *it.second()
    };
    assert!(((x3) == (4_u32)));
    let mut x4: u32 = if const_it == UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<u32>>) {
        0_u32
    } else {
        *const_it.second()
    };
    assert!(((x4) == (0_u32)));
    (*m.entry(4_i16).or_default().as_mut()) = 5_u32;
    let mut it4: UnsafeMapIterator<i16, u32> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<u32>>, &4_i16);
    let mut p: *mut u32 = (&mut *it4.second() as *mut u32);
    let mut x5: u32 = (*p);
    assert!(((*m.entry(4_i16).or_default().as_mut()) == (5_u32)));
    assert!(((*it4.second()) == (5_u32)));
    assert!(((*p) == (5_u32)));
    assert!(((x5) == (5_u32)));
    (*p).prefix_inc();
    assert!(((*m.entry(4_i16).or_default().as_mut()) == (6_u32)));
    assert!(((*it4.second()) == (6_u32)));
    assert!(((*p) == (6_u32)));
    assert!(((x5) == (5_u32)));
    let r: *mut BTreeMap<i16, Box<u32>> = &mut m as *mut BTreeMap<i16, Box<u32>>;
    assert!((((*r).len() as u64) == (4_u64)));
    assert!(
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<u32>>, &4_i16)
            != UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<u32>>)
    );
    UnsafeMapIterator::erase(&(*r) as *const BTreeMap<i16, Box<u32>>, &it4.clone());
    assert!((((*r).len() as u64) == (3_u64)));
    assert!(
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i16, Box<u32>>, &4_i16)
            == UnsafeMapIterator::end(&m as *const BTreeMap<i16, Box<u32>>)
    );
    let mut other_map: BTreeMap<(i32, i64), Box<f64>> = BTreeMap::new();
    assert!(((other_map.len() as u64) == (0_u64)));
    let mut key0: (i32, i64) = (1.into(), 1.into());
    let mut value: f64 = 2_f64;
    (*other_map.entry(key0).or_default().as_mut()) = value;
    value = (*other_map.entry(key0).or_default().as_mut());
    assert!(((other_map.len() as u64) == (1_u64)));
    assert!(((*other_map.entry(key0).or_default().as_mut()) == (value)));
    assert!(((m.len() as u64) == (3_u64)));
    let mut k: i32 = 0;
    assert!(((*(m.get(&(k as i16)).expect("out of range!").as_ref() as *const u32)) == (5_u32)));
    k.prefix_inc();
    assert!(((*(m.get(&(k as i16)).expect("out of range!").as_ref() as *const u32)) == (4_u32)));
    k.prefix_inc();
    assert!(((*(m.get(&(k as i16)).expect("out of range!").as_ref() as *const u32)) == (4_u32)));
    let mut m2: BTreeMap<i32, Box<bool>> = BTreeMap::new();
    assert!(((m2.len() as u64) == (0_u64)));
    let mut indexes: Vec<i32> = Vec::new();
    let mut i: u32 = 60_u32;
    'loop_: while ((i) > (30_u32)) {
        indexes.push((i as i32));
        i.prefix_dec();
    }
    let mut i: u32 = 100_u32;
    'loop_: while ((i) > (60_u32)) {
        indexes.push((i as i32));
        i.prefix_dec();
    }
    let mut i: u32 = 30_u32;
    'loop_: while ((i) > (0_u32)) {
        indexes.push((i as i32));
        i.prefix_dec();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (indexes.len() as u64)) {
        (*m2.entry(indexes[(i as u64) as usize]).or_default().as_mut()) =
            ((i).wrapping_rem(2_u32) != 0);
        i.prefix_inc();
    }
    assert!(((m2.len() as u64) == (indexes.len() as u64)));
    let mut last: i32 = -1_i32;
    'loop_: for pair in UnsafeMapIterator::begin(&m2 as *const BTreeMap<i32, Box<bool>>) {
        assert!(((*pair.first()) > (last)));
        assert!(((*pair.second() as i32) == ((*pair.first()) % (2))));
        last = *pair.first();
    }
    k = 0;
    let value_0: *const u32 = (m.get(&(k as i16)).expect("out of range!").as_ref() as *const u32);
    return ((((((((m.len() as u64).wrapping_add((x1 as u64))).wrapping_add((x2 as u64)))
        .wrapping_add((x3 as u64)))
    .wrapping_add((x4 as u64)))
    .wrapping_add((x5 as u64)))
    .wrapping_add(((*value_0) as u64))) as i32);
}
