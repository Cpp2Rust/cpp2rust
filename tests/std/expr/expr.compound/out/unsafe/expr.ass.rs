extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn next_0(mut pp: *mut *mut i32, calls: *mut i32) -> *mut *mut i32 {
    (*calls).prefix_inc();
    return pp;
}
pub unsafe fn test_1() {
    let mut a: i32 = 1;
    let mut b: i32 = 42;
    let r: *mut i32 = &mut ({
        a = b;
        a
    });
    assert!(((r) == (&mut a as *mut i32)));
    (*(match ({ b }, (&mut a as *mut i32)) {
        (__tmp_1, __tmp_0) => {
            __tmp_0.write(__tmp_1);
            __tmp_0
        }
    })) = 5;
    assert!(((a) == (5)));
    assert!(((b) == (42)));
    (*(match ({ 2 }, (&mut a as *mut i32)) {
        (__tmp_3, __tmp_2) => {
            __tmp_2.write((((*__tmp_2) as i32) + (__tmp_3 as i32)) as i32);
            __tmp_2
        }
    })) *= 3;
    assert!(((a) == (21)));
    let mut c: i32 = 0;
    (*(match ({ b }, (&mut c as *mut i32)) {
        (__tmp_5, __tmp_4) => {
            __tmp_4.write(__tmp_5);
            __tmp_4
        }
    })) = 3;
    assert!(((c) == (3)));
    (*(match ({ 1 }, (&mut c as *mut i32)) {
        (__tmp_7, __tmp_6) => {
            __tmp_6.write((((*__tmp_6) as i32) - (__tmp_7 as i32)) as i32);
            __tmp_6
        }
    })) += 10;
    assert!(((c) == (12)));
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut i: i32 = 0;
    (*(match ({ 9 }, (&mut arr[(i.postfix_inc()) as usize] as *mut i32)) {
        (__tmp_9, __tmp_8) => {
            __tmp_8.write(__tmp_9);
            __tmp_8
        }
    })) = 7;
    assert!(((i) == (1)));
    assert!(((arr[(0) as usize]) == (7)));
    assert!(((arr[(1) as usize]) == (2)));
    let mut p: *mut i32 = arr.as_mut_ptr();
    (match (
        { 1 },
        ((match ({ 1 }, (&mut p as *mut *mut i32)) {
            (__tmp_11, __tmp_10) => {
                __tmp_10.write((*__tmp_10).offset((__tmp_11 as isize)));
                __tmp_10
            }
        }) as *mut *mut i32),
    ) {
        (__tmp_13, __tmp_12) => {
            __tmp_12.write((*__tmp_12).offset((__tmp_13 as isize)));
            __tmp_12
        }
    });
    assert!(((p) == (arr.as_mut_ptr().offset((2) as isize))));
    let mut q: *mut i32 = arr.as_mut_ptr();
    let mut calls: i32 = 0;
    (match (
        { 1 },
        ((unsafe {
            let _pp: *mut *mut i32 = (&mut q as *mut *mut i32);
            let _calls: *mut i32 = &mut calls;
            next_0(_pp, _calls)
        }) as *mut *mut i32),
    ) {
        (__tmp_15, __tmp_14) => {
            __tmp_14.write((*__tmp_14).offset((__tmp_15 as isize)));
            __tmp_14
        }
    });
    assert!(((calls) == (1)));
    assert!(((q) == (arr.as_mut_ptr().offset((1) as isize))));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
