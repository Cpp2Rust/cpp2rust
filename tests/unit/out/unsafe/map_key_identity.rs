extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut default_constructions_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Key {
    pub order: i32,
    pub identity: i32,
}
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Key, other as *const Key) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Key, self as *const Key) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const Key, other as *const Key) }
    }
}
impl std::cmp::Eq for Key {}
pub unsafe fn operator_lt_1(a: *const Key, b: *const Key) -> bool {
    return (((*a).order) < ((*b).order));
}
pub unsafe fn operator_eq_2(a: *const Key, b: *const Key) -> bool {
    return (((*a).identity) == ((*b).identity));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Mapped {
    pub value: i32,
}
impl Mapped {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)).prefix_inc();
        this
    }
}
impl Default for Mapped {
    fn default() -> Self {
        unsafe { Mapped::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = true;
    let distinct_a: Key = Key {
        order: 1,
        identity: 7,
    };
    let distinct_b: Key = Key {
        order: 2,
        identity: 7,
    };
    ok = ((ok as i32)
        & ((unsafe {
            let _a: *const Key = &distinct_a;
            operator_eq_2(_a, &distinct_b)
        }) as i32))
        != 0;
    let mut distinct: UnsafeMap<Key, (Key, Mapped)> = UnsafeMap::new();
    let a: *mut Mapped = &mut (*&mut (*distinct
        .get_or_insert_with(distinct_a.clone(), || (distinct_a, <Mapped>::default())))
    .1);
    (*a).value = 11;
    let mut a_it: UnsafeBidirectionalIterator<(Key, Mapped)> = distinct.find(&distinct_a);
    let mut a_value: *mut Mapped = (&mut (*a_it.iter_deref()).1 as *mut Mapped);
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)) == (1))
            as i32))
        != 0;
    let b: *mut Mapped = &mut (*&mut (*distinct
        .get_or_insert_with(distinct_b.clone(), || (distinct_b, <Mapped>::default())))
    .1);
    (*b).value = 22;
    let mut b_it: UnsafeBidirectionalIterator<(Key, Mapped)> = distinct.find(&distinct_b);
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0)) == (2))
            as i32))
        != 0;
    ok = ((ok as i32) & (((distinct.len()) == (2_usize)) as i32)) != 0;
    ok = ((ok as i32) & (((a) != (b)) as i32)) != 0;
    ok = ((ok as i32) & (((((*a).value) == (11)) && (((*b).value) == (22))) as i32)) != 0;
    ok = ((ok as i32) & ((a_it == distinct.find(&distinct_a)) as i32)) != 0;
    ok = ((ok as i32) & ((b_it == distinct.find(&distinct_b)) as i32)) != 0;
    ok = ((ok as i32) & ((b_it != distinct.end()) as i32)) != 0;
    ok = ((ok as i32) & (((&mut (*a_it.iter_deref()).1 as *mut Mapped) == (a_value)) as i32)) != 0;
    ok = ((ok as i32) & ((((*a_it.iter_deref()).1.value) == (11)) as i32)) != 0;
    ok = ((ok as i32)
        & ((unsafe {
            let _a: *const Key = &distinct_a;
            operator_eq_2(_a, &distinct_b)
        }) as i32))
        != 0;
    let mut distinct_count: usize = 0_usize;
    'loop_: for entry in UnsafeIterRange::new(distinct.begin(), distinct.end()) {
        let entry: *const (Key, Mapped) = &(*entry.iter_deref());
        if ((distinct_count) == (0_usize)) {
            ok = ((ok as i32)
                & (((((*entry).0.order) == (1)) && (((*entry).1.value) == (11))) as i32))
                != 0;
        } else if ((distinct_count) == (1_usize)) {
            ok = ((ok as i32)
                & (((((*entry).0.order) == (2)) && (((*entry).1.value) == (22))) as i32))
                != 0;
        }
        distinct_count.prefix_inc();
    }
    ok = ((ok as i32) & (((distinct_count) == (2_usize)) as i32)) != 0;
    let equivalent_a: Key = Key {
        order: 3,
        identity: 8,
    };
    let equivalent_b: Key = Key {
        order: 3,
        identity: 9,
    };
    ok = ((ok as i32)
        & (!(unsafe {
            let _a: *const Key = &equivalent_a;
            operator_eq_2(_a, &equivalent_b)
        }) as i32))
        != 0;
    let mut equivalent: UnsafeMap<Key, (Key, Mapped)> = UnsafeMap::new();
    let before_equivalent: i32 =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0));
    let first: *mut Mapped = &mut (*&mut (*equivalent
        .get_or_insert_with(equivalent_a.clone(), || (equivalent_a, <Mapped>::default())))
    .1);
    (*first).value = 31;
    let mut first_it: UnsafeBidirectionalIterator<(Key, Mapped)> = equivalent.find(&equivalent_a);
    let mut first_value: *mut Mapped = (&mut (*first_it.iter_deref()).1 as *mut Mapped);
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
            == ((before_equivalent) + (1))) as i32))
        != 0;
    let second: *mut Mapped = &mut (*&mut (*equivalent
        .get_or_insert_with(equivalent_b.clone(), || (equivalent_b, <Mapped>::default())))
    .1);
    let reused: bool = ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
        == ((before_equivalent) + (1)));
    ok = ((ok as i32) & (reused as i32)) != 0;
    ok = ((ok as i32) & (((equivalent.len()) == (1_usize)) as i32)) != 0;
    if reused {
        ok = ((ok as i32) & ((first_it != equivalent.end()) as i32)) != 0;
        ok = ((ok as i32) & (((first) == (second)) as i32)) != 0;
        ok = ((ok as i32) & ((first_it == equivalent.find(&equivalent_a)) as i32)) != 0;
        ok = ((ok as i32) & ((first_it == equivalent.find(&equivalent_b)) as i32)) != 0;
        ok = ((ok as i32)
            & (((&mut (*first_it.iter_deref()).1 as *mut Mapped) == (first_value)) as i32))
            != 0;
        ok = ((ok as i32)
            & (((((*first_it.iter_deref()).1.value) == (31)) && (((*second).value) == (31)))
                as i32))
            != 0;
        let mut equivalent_count: usize = 0_usize;
        'loop_: for entry in UnsafeIterRange::new(equivalent.begin(), equivalent.end()) {
            let entry: *const (Key, Mapped) = &(*entry.iter_deref());
            ok = ((ok as i32)
                & (((((*entry).0.order) == (3)) && (((*entry).1.value) == (31))) as i32))
                != 0;
            equivalent_count.prefix_inc();
        }
        ok = ((ok as i32) & (((equivalent_count) == (1_usize)) as i32)) != 0;
    }
    ok = ((ok as i32)
        & (!(unsafe {
            let _a: *const Key = &equivalent_a;
            operator_eq_2(_a, &equivalent_b)
        }) as i32))
        != 0;
    let later_key: Key = Key {
        order: 4,
        identity: 10,
    };
    let later: *mut Mapped = &mut (*&mut (*equivalent
        .get_or_insert_with(later_key.clone(), || (later_key, <Mapped>::default())))
    .1);
    (*later).value = 41;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
            == ((before_equivalent) + (2))) as i32))
        != 0;
    ok = ((ok as i32) & (((equivalent.len()) == (2_usize)) as i32)) != 0;
    if reused {
        ok = ((ok as i32) & ((first_it == equivalent.find(&equivalent_a)) as i32)) != 0;
        ok = ((ok as i32)
            & (((&mut (*first_it.iter_deref()).1 as *mut Mapped) == (first_value)) as i32))
            != 0;
        ok = ((ok as i32) & ((((*first_it.iter_deref()).1.value) == (31)) as i32)) != 0;
    }
    let matching_a: Key = Key {
        order: 5,
        identity: 5,
    };
    let matching_b: Key = Key {
        order: 5,
        identity: 5,
    };
    let matching_c: Key = Key {
        order: 6,
        identity: 6,
    };
    let before_matching: i32 =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0));
    let mut matching: UnsafeMap<Key, (Key, Mapped)> = UnsafeMap::new();
    let matched: *mut Mapped = &mut (*&mut (*matching
        .get_or_insert_with(matching_a.clone(), || (matching_a, <Mapped>::default())))
    .1);
    (*matched).value = 51;
    let mut matched_it: UnsafeBidirectionalIterator<(Key, Mapped)> = matching.find(&matching_a);
    let mut matched_value: *mut Mapped = (&mut (*matched_it.iter_deref()).1 as *mut Mapped);
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
            == ((before_matching) + (1))) as i32))
        != 0;
    let matched_again: *mut Mapped = &mut (*&mut (*matching
        .get_or_insert_with(matching_b.clone(), || (matching_b, <Mapped>::default())))
    .1);
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
            == ((before_matching) + (1))) as i32))
        != 0;
    ok = ((ok as i32) & (((matching.len()) == (1_usize)) as i32)) != 0;
    ok = ((ok as i32) & (((matched) == (matched_again)) as i32)) != 0;
    ok = ((ok as i32) & ((matched_it == matching.find(&matching_b)) as i32)) != 0;
    ok = ((ok as i32)
        & (((&mut (*matched_it.iter_deref()).1 as *mut Mapped) == (matched_value)) as i32))
        != 0;
    ok = ((ok as i32) & ((((*matched_it.iter_deref()).1.value) == (51)) as i32)) != 0;
    let other: *mut Mapped = &mut (*&mut (*matching
        .get_or_insert_with(matching_c.clone(), || (matching_c, <Mapped>::default())))
    .1);
    (*other).value = 61;
    ok = ((ok as i32)
        & (((*std::cell::LazyCell::force_mut(&mut *&raw mut default_constructions_0))
            == ((before_matching) + (2))) as i32))
        != 0;
    ok = ((ok as i32) & (((matching.len()) == (2_usize)) as i32)) != 0;
    ok = ((ok as i32) & ((matched_it == matching.find(&matching_a)) as i32)) != 0;
    ok = ((ok as i32)
        & (((&mut (*matched_it.iter_deref()).1 as *mut Mapped) == (matched_value)) as i32))
        != 0;
    ok = ((ok as i32) & ((((*matched_it.iter_deref()).1.value) == (51)) as i32)) != 0;
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_constructions_0);
}
