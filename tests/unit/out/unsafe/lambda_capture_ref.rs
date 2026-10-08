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
pub struct S {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    let mut add_base: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let base: *mut i32 = &mut base;
        },
        |x: i32| -> i32 {
            return ((x) + (*base));
        },
        copy_from { base: base },
        move_from { base: base }
    );
    assert!(((unsafe { add_base.call(5,) }) == (15)));
    base = 100;
    assert!(((unsafe { add_base.call(5,) }) == (105)));
    let mut s: S = S { x: 1, y: 2 };
    let mut sum: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let s: *mut S = &mut s;
        },
        || -> i32 {
            return (((*s).x) + ((*s).y));
        },
        copy_from { s: s },
        move_from { s: s }
    );
    assert!(((unsafe { sum.call() }) == (3)));
    s.x = 50;
    assert!(((unsafe { sum.call() }) == (52)));
    let mut counter: i32 = 0;
    let mut bump: FnPtr<fn()> = lambda_unsafe!(
        {
            let counter: *mut i32 = &mut counter;
        },
        || {
            (*counter).postfix_inc();
        },
        copy_from { counter: counter },
        move_from { counter: counter }
    );
    (unsafe { bump.call() });
    (unsafe { bump.call() });
    assert!(((counter) == (2)));
    let mut arr: [u16; 4] = [3_u16, 1_u16, 2_u16, 0_u16];
    let mut swap: FnPtr<fn(usize, usize)> = lambda_unsafe!(
        {
            let arr: *mut [u16; 4] = &mut arr;
        },
        |i: usize, j: usize| {
            let mut t: u16 = (*arr)[(j)];
            (*arr)[(j)] = (*arr)[(i)];
            (*arr)[(i)] = t;
        },
        copy_from { arr: arr },
        move_from { arr: arr }
    );
    (unsafe { swap.call(0_usize, 3_usize) });
    assert!(((arr[(0) as usize] as i32) == (0)));
    assert!(((arr[(3) as usize] as i32) == (3)));
    let mut total: i32 = 0;
    let mut add: FnPtr<fn(i32)> = lambda_unsafe!(
        {
            let t: *mut i32 = &mut total;
        },
        |x: i32| {
            (*t) += x;
        },
        copy_from { t: t },
        move_from { t: t }
    );
    (unsafe { add.call(2) });
    (unsafe { add.call(3) });
    assert!(((total) == (5)));
    let mut set_y: FnPtr<fn(i32)> = lambda_unsafe!(
        {
            let y: *mut i32 = &mut s.y;
        },
        |v: i32| {
            (*y) = v;
        },
        copy_from { y: y },
        move_from { y: y }
    );
    (unsafe { set_y.call(9) });
    assert!(((s.y) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
