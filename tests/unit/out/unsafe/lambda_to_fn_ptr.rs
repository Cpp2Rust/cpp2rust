extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut x: i32, mut fn_: Option<unsafe fn(i32) -> i32>) -> i32 {
    return (unsafe { (fn_).unwrap()(x) });
}
pub unsafe fn call_nttp_1(mut x: i32) -> i32 {
    return (unsafe { (Some(int_operator___int__const::__invoke))(x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fresh: Option<unsafe fn(i32) -> i32> = Some(|x: i32| {
        return -x;
    });
    assert!(((unsafe { (fresh).unwrap()(5,) }) == (-5_i32)));
    let mut named: Option<unsafe fn(i32) -> i32> = (unsafe {
        (|x: i32| {
            return ((x) * (2));
        })()
    });
    assert!(((unsafe { (named).unwrap()(5,) }) == (10)));
    assert!(
        ((unsafe {
            apply_0(
                5,
                (unsafe {
                    (|x: i32| {
                        return ((x) * (2));
                    })()
                }),
            )
        }) == (10))
    );
    named = fresh;
    assert!(((unsafe { (named).unwrap()(3,) }) == (-3_i32)));
    let mut p: Option<unsafe fn(i32) -> i32> = (unsafe {
        (|x: i32| {
            return ((x) * (3));
        })()
    });
    assert!(((unsafe { (p).unwrap()(2,) }) == (6)));
    assert!(((unsafe { call_nttp_1(5,) }) == (4)));
    assert!(
        ((unsafe {
            (|n: i32| {
                let mut ap: VaList = VaList::default();
                ap = VaList::new(__args);
                let mut s: i32 = 0;
                let mut i: i32 = 0;
                'loop_: while ((i) < (n)) {
                    s += ap.arg::<i32>();
                    i.postfix_inc();
                }
                return s;
            })(2, &[(5).into(), (5).into()])
        }) == (10))
    );
    assert!(
        ((unsafe {
            (|n: i32| {
                let mut ap: VaList = VaList::default();
                ap = VaList::new(__args);
                let mut s: i32 = 0;
                let mut i: i32 = 0;
                'loop_: while ((i) < (n)) {
                    s += ap.arg::<i32>();
                    i.postfix_inc();
                }
                return s;
            })(3, &[(1).into(), (2).into(), (3).into()])
        }) == (6))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
