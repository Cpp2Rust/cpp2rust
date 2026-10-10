extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fact: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut self_: FnPtr<Generic>,
                                                                mut n: i32|
     -> i32 {
        if ((n) <= (1)) {
            return 1;
        }
        return ((n)
            * (unsafe {
                let _self_: FnPtr<Generic> = self_.copy_from();
                self_
                    .spec::<fn(FnPtr<Generic>, i32) -> i32>(0)
                    .call(_self_, ((n) - (1)))
            }));
    });
    assert!(
        ((unsafe {
            let _self_: FnPtr<Generic> = fact.copy_from();
            fact.spec::<fn(FnPtr<Generic>, i32) -> i32>(0)
                .call(_self_, 5)
        }) == (120))
    );
    let mut calls: i32 = 0;
    let mut fib: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let calls: *mut i32 = &mut calls;
        },
        |self_: *mut FnPtr<Generic>, mut n: i32| -> i32 {
            (*calls).postfix_inc();
            if ((n) <= (2)) {
                return 1;
            }
            return ((unsafe {
                let _self_: *mut FnPtr<Generic> = self_;
                let _n: i32 = ((n) - (1));
                (*self_)
                    .spec::<fn(*mut FnPtr<Generic>, i32) -> i32>(0)
                    .call(_self_, _n)
            }) + (unsafe {
                let _self_: *mut FnPtr<Generic> = self_;
                let _n: i32 = ((n) - (2));
                (*self_)
                    .spec::<fn(*mut FnPtr<Generic>, i32) -> i32>(0)
                    .call(_self_, _n)
            }));
        }
    );
    assert!(
        ((unsafe {
            let _self_: *mut FnPtr<Generic> = &mut fib;
            fib.spec::<fn(*mut FnPtr<Generic>, i32) -> i32>(0)
                .call(_self_, 6)
        }) == (8))
    );
    assert!(((calls) == (15)));
    let mut depth: i32 = 0;
    let mut count_down: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let depth: *mut i32 = &mut depth;
        },
        |self_: *const FnPtr<Generic>, mut n: i32| {
            if ((n) == (0)) {
                return;
            }
            (*depth).postfix_inc();
            (unsafe {
                let _self_: *const FnPtr<Generic> = self_;
                let _n: i32 = ((n) - (1));
                (*self_)
                    .spec::<fn(*const FnPtr<Generic>, i32)>(0)
                    .call(_self_, _n)
            });
        }
    );
    (unsafe {
        let _self_: *const FnPtr<Generic> = &count_down;
        count_down
            .spec::<fn(*const FnPtr<Generic>, i32)>(0)
            .call(_self_, 4)
    });
    assert!(((depth) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
