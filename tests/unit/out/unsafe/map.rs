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
    let mut m: UnsafeMap<i16, (i16, u32)> = UnsafeMap::new();
    (*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k))) = 1_u32;
    (*m.subscript_move(&mut 1_i16, |__k| cpp_move_unsafe!(__k))) = 2_u32;
    (*m.subscript_move(&mut 2_i16, |__k| cpp_move_unsafe!(__k))) = 3_u32;
    assert!(((m.len()) == (3_usize)));
    assert!(((*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k))) == (1_u32)));
    assert!(((*m.subscript_move(&mut 1_i16, |__k| cpp_move_unsafe!(__k))) == (2_u32)));
    assert!(((*m.subscript_move(&mut 2_i16, |__k| cpp_move_unsafe!(__k))) == (3_u32)));
    let mut x: i32 = 4;
    (*m.subscript_move(&mut 1_i16, |__k| cpp_move_unsafe!(__k))) = (x as u32);
    assert!(((m.len()) == (3_usize)));
    assert!(((*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k))) == (1_u32)));
    assert!(((*m.subscript_move(&mut 1_i16, |__k| cpp_move_unsafe!(__k))) == (4_u32)));
    assert!(((*m.subscript_move(&mut 2_i16, |__k| cpp_move_unsafe!(__k))) == (3_u32)));
    (unsafe { foo_0((*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k)))) });
    assert!(((*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k))) == (1_u32)));
    (unsafe { bar_1(&mut (*m.subscript_move(&mut 2_i16, |__k| cpp_move_unsafe!(__k)))) });
    assert!(((*m.subscript_move(&mut 2_i16, |__k| cpp_move_unsafe!(__k))) == (4_u32)));
    (*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k))) = (*m
        .subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k)))
    .wrapping_add((*m.subscript_move(&mut 2_i16, |__k| cpp_move_unsafe!(__k))));
    assert!(((*m.subscript_move(&mut 0_i16, |__k| cpp_move_unsafe!(__k))) == (5_u32)));
    let mut end: UnsafeBidirectionalIterator<(i16, u32)> = m.end();
    let mut it: UnsafeBidirectionalIterator<(i16, u32)> = m.find(&1_i16);
    let mut const_it: UnsafeBidirectionalIterator<(i16, u32)> =
        <UnsafeBidirectionalIterator<(i16, u32)>>::from(m.find(&10_i16));
    let mut x1: u32 = if it == end {
        0_u32
    } else {
        (*it.iter_deref()).1
    };
    assert!(((x1) == (4_u32)));
    let mut x2: u32 = if const_it == <UnsafeBidirectionalIterator<(i16, u32)>>::from((end).clone())
    {
        0_u32
    } else {
        (*const_it.iter_deref()).1
    };
    assert!(((x2) == (0_u32)));
    let mut x3: u32 = if it == m.end() {
        0_u32
    } else {
        (*it.iter_deref()).1
    };
    assert!(((x3) == (4_u32)));
    let mut x4: u32 = if const_it == <UnsafeBidirectionalIterator<(i16, u32)>>::from(m.end()) {
        0_u32
    } else {
        (*const_it.iter_deref()).1
    };
    assert!(((x4) == (0_u32)));
    (*m.subscript_move(&mut 4_i16, |__k| cpp_move_unsafe!(__k))) = 5_u32;
    let mut it4: UnsafeBidirectionalIterator<(i16, u32)> = m.find(&4_i16);
    let mut p: *mut u32 = (&mut (*it4.iter_deref()).1 as *mut u32);
    let mut x5: u32 = (*p);
    assert!(((*m.subscript_move(&mut 4_i16, |__k| cpp_move_unsafe!(__k))) == (5_u32)));
    assert!((((*it4.iter_deref()).1) == (5_u32)));
    assert!(((*p) == (5_u32)));
    assert!(((x5) == (5_u32)));
    (*p).prefix_inc();
    assert!(((*m.subscript_move(&mut 4_i16, |__k| cpp_move_unsafe!(__k))) == (6_u32)));
    assert!((((*it4.iter_deref()).1) == (6_u32)));
    assert!(((*p) == (6_u32)));
    assert!(((x5) == (5_u32)));
    let mut const_it4: UnsafeBidirectionalIterator<(i16, u32)> =
        <UnsafeBidirectionalIterator<(i16, u32)>>::from(m.find(&4_i16));
    assert!((((*const_it4.iter_deref()).1) == (6_u32)));
    let r: *mut UnsafeMap<i16, (i16, u32)> = &mut m;
    assert!((((*r).len()) == (4_usize)));
    assert!(m.find(&4_i16) != m.end());
    (*r).erase(&(it4).clone());
    assert!((((*r).len()) == (3_usize)));
    assert!(m.find(&4_i16) == m.end());
    (*m.subscript_move(&mut 7_i16, |__k| cpp_move_unsafe!(__k))) = 7_u32;
    (*m.subscript_move(&mut 8_i16, |__k| cpp_move_unsafe!(__k))) = 8_u32;
    (*m.subscript_move(&mut 9_i16, |__k| cpp_move_unsafe!(__k))) = 9_u32;
    assert!((((*r).len()) == (6_usize)));
    assert!(((m.erase_key(&10_i16)) == (0_usize)));
    let mut key9: i32 = 9;
    assert!(((m.erase_key(&(key9 as i16))) == (1_usize)));
    assert!((((*r).len()) == (5_usize)));
    let mut cit: UnsafeBidirectionalIterator<(i16, u32)> =
        <UnsafeBidirectionalIterator<(i16, u32)>>::from(m.find(&7_i16));
    let mut next: UnsafeBidirectionalIterator<(i16, u32)> = m.erase(&(cit).clone());
    assert!(next == m.find(&8_i16));
    next = m.erase(&(next).clone());
    assert!(next == m.end());
    assert!((((*r).len()) == (3_usize)));
    assert!(m.find(&8_i16) == m.end());
    let mut other_map: UnsafeMap<(i32, i64), ((i32, i64), f64)> = UnsafeMap::new();
    assert!(((other_map.len()) == (0_usize)));
    let mut key0: (i32, i64) = (1.into(), 1.into());
    let mut value: f64 = 2_f64;
    (*other_map.subscript(&key0)) = value;
    value = (*other_map.subscript(&key0));
    assert!(((other_map.len()) == (1_usize)));
    assert!(((*other_map.subscript(&key0)) == (value)));
    assert!(((m.len()) == (3_usize)));
    let mut k: i32 = 0;
    assert!((((*m.get(&(k as i16)).expect("out of range!")).1) == (5_u32)));
    k.prefix_inc();
    assert!((((*m.get(&(k as i16)).expect("out of range!")).1) == (4_u32)));
    k.prefix_inc();
    assert!((((*m.get(&(k as i16)).expect("out of range!")).1) == (4_u32)));
    let mut m2: UnsafeMap<i32, (i32, bool)> = UnsafeMap::new();
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
        (*m2.subscript(&indexes[(i as usize)])) = ((i).wrapping_rem(2_u32) != 0);
        i.prefix_inc();
    }
    assert!(((m2.len()) == (indexes.len())));
    let mut last: i32 = -1_i32;
    'loop_: for pair in IterRange::new(m2.begin(), m2.end()) {
        let pair: *mut (i32, bool) = &mut (*pair.iter_deref());
        assert!((((*pair).0) > (last)));
        assert!((((*pair).1 as i32) == (((*pair).0) % (2))));
        last = (*pair).0;
    }
    k = 0;
    let value_0: *const u32 = &raw const (*m.get(&(k as i16)).expect("out of range!")).1;
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
