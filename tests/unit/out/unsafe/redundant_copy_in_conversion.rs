extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sink_0(mut it: UnsafeBidirectionalIterator<(i32, i32)>) -> i32 {
    let mut cit: UnsafeBidirectionalIterator<(i32, i32)> =
        std__Rb_tree_const_iterator_std_pair_const_int__int__::new_1({ &it });
    return if cit == std__Rb_tree_const_iterator_std_pair_const_int__int__::new_1({ &it }) {
        (*it.iter_deref()).second
    } else {
        0
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: BTreeMap<i32, Box<(i32, i32)>> = BTreeMap::new();
    (*&mut m
        .entry(0.clone())
        .or_insert_with(|| Box::new((0, <i32>::default())))
        .1) = 1;
    let mut end: UnsafeBidirectionalIterator<(i32, i32)> =
        UnsafeMapIterator::end(&m as *const BTreeMap<i32, Box<(i32, i32)>>);
    let mut it0: UnsafeBidirectionalIterator<(i32, i32)> =
        UnsafeMapIterator::find_key(&m as *const BTreeMap<i32, Box<(i32, i32)>>, &0);
    let mut const_it: UnsafeBidirectionalIterator<(i32, i32)> =
        std__Rb_tree_const_iterator_std_pair_const_int__int__::new_1({ &it0 });
    let mut r: i32 =
        if const_it == std__Rb_tree_const_iterator_std_pair_const_int__int__::new_1({ &end }) {
            0
        } else {
            1
        };
    r += (unsafe { sink_0(it0.clone()) });
    r += if end == end { 0 } else { 1 };
    assert!(((r) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
