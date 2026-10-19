extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Pair; 3] = [
        Pair {
            first: 1,
            second: 2,
        },
        Pair {
            first: 3,
            second: 4,
        },
        Pair {
            first: 5,
            second: 6,
        },
    ];
    'loop_: for __decomp_0 in
        IterRange::new(arr.as_mut_ptr(), arr.as_mut_ptr().offset((3_i64) as isize))
    {
        let __decomp_0: *mut Pair = &mut (*__decomp_0);
        (*__decomp_0).first += (*__decomp_0).second;
    }
    assert!(((arr[(0) as usize].first) == (3)));
    assert!(((arr[(2) as usize].first) == (11)));
    'loop_: for __decomp_1 in
        IterRange::new(arr.as_mut_ptr(), arr.as_mut_ptr().offset((3_i64) as isize))
    {
        let mut __decomp_1: Pair = (*__decomp_1);
        __decomp_1.first = 0;
        __decomp_1.second = 0;
    }
    assert!(((arr[(1) as usize].first) == (7)));
    assert!(((arr[(1) as usize].second) == (4)));
    let mut sum: i32 = 0;
    'loop_: for __decomp_2 in
        IterRange::new(arr.as_mut_ptr(), arr.as_mut_ptr().offset((3_i64) as isize))
    {
        let __decomp_2: *const Pair = &(*__decomp_2);
        sum += (((*__decomp_2).first) * ((*__decomp_2).second));
    }
    assert!(((sum) == ((((3) * (2)) + ((7) * (4))) + ((11) * (6)))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
