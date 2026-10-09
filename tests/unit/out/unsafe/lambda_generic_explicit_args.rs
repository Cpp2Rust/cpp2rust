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
pub struct Val {
    pub x: i32,
}
pub unsafe fn sum_0(mut a: Val, mut b: Val) -> i32 {
    return ((a.x) + (b.x));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut total: i32 = 0;
    let mut tally: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let total: *mut i32 = &mut total;
        },
        || {
            (*total) = (((*total) as usize).wrapping_add(
                ((::std::mem::size_of::<libc::c_char>() as usize)
                    .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize))
                    as usize),
            )) as i32;
        },
        || {
            (*total) = (((*total) as usize).wrapping_add(
                ((::std::mem::size_of::<i32>() as usize)
                    .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize))
                    as usize),
            )) as i32;
        }
    );
    (unsafe { tally.spec::<fn()>(0).call() });
    (unsafe { tally.spec::<fn()>(1).call() });
    assert!(((total) == (7)));
    let mut v: Val = Val { x: 5 };
    let mut acc: i32 = 0;
    let mut pick: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let v: *mut Val = &mut v;
            let acc: *mut i32 = &mut acc;
        },
        || {
            (*acc) += (unsafe {
                let _a: Val = (*v);
                let _b: Val = (*v);
                sum_0(_a, _b)
            });
        },
        || {
            (*acc) += (unsafe {
                let _a: Val = (*v);
                let _b: Val = (*v);
                sum_0(_a, _b)
            });
        },
        || {
            (*acc) += (unsafe {
                let _a: Val = (*v);
                let _b: Val = (*v);
                sum_0(_a, _b)
            });
        }
    );
    (unsafe { pick.spec::<fn()>(0).call() });
    (unsafe { pick.spec::<fn()>(1).call() });
    (unsafe { pick.spec::<fn()>(2).call() });
    assert!(((acc) == (30)));
    let mut cast_to: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |x: i32| -> i32 {
            return ((x as i32) / (2));
        },
        |x: i32| -> f64 {
            return ((x as f64) / (2_f64));
        }
    );
    assert!(((unsafe { cast_to.spec::<fn(i32) -> i32>(0).call(5,) }) == (2)));
    assert!(((unsafe { cast_to.spec::<fn(i32) -> f64>(1).call(5,) }) == (2.5_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
