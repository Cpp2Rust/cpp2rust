extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut counter_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn apply_1(mut f: impl Fn(i32) -> i32, mut x: i32) -> i32 {
    return (unsafe { f(x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) + (1));
            })(41)
        }) == (42))
    );
    (unsafe {
        (|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)).postfix_inc();
            return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0));
        })()
    });
    assert!(
        ((unsafe {
            (|| {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)).postfix_inc();
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0));
            })()
        }) == (2))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_0)) == (2)));
    assert!(
        ((unsafe {
            apply_1(
                (|x: i32| {
                    return ((x) + (1));
                })
                .clone(),
                1,
            )
        }) == (2))
    );
    let mut copy: _ = (|x: i32| {
        return ((x) + (1));
    })
    .clone();
    assert!(((unsafe { copy(9,) }) == (10)));
    let mut fp: Option<unsafe fn(i32) -> i32> = (unsafe {
        (|x: i32| {
            return ((x) + (1));
        })()
    });
    assert!(((unsafe { (fp).unwrap()(-1_i32,) }) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_0);
}
