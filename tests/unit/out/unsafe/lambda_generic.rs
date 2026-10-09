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
    let mut twice: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: i32| -> i32 {
            return ((x) + (x));
        },
        |mut x: f64| -> f64 {
            return ((x) + (x));
        }
    );
    assert!(((unsafe { twice.spec::<fn(i32) -> i32>(0).call(4,) }) == (8)));
    assert!(((unsafe { twice.spec::<fn(f64) -> f64>(1).call(1.5_f64,) }) == (3_f64)));
    let mut base: i32 = 10;
    let mut add_base: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let base: i32 = base;
        },
        |mut x: i32| -> i32 {
            return ((x) + (base));
        },
        |mut x: f64| -> f64 {
            return ((x) + (base as f64));
        }
    );
    assert!(((unsafe { add_base.spec::<fn(i32) -> i32>(0).call(5,) }) == (15)));
    assert!(((unsafe { add_base.spec::<fn(f64) -> f64>(1).call(2.5_f64,) }) == (12.5_f64)));
    let mut total: i32 = 0;
    let mut accumulate: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let total: *mut i32 = &mut total;
        },
        |mut x: i32, mut y: i32| {
            (*total) += ((x) * (y));
        },
        |mut x: u32, mut y: u32| {
            (*total) = (((*total) as u32).wrapping_add((x).wrapping_mul(y))) as i32;
        }
    );
    (unsafe { accumulate.spec::<fn(i32, i32)>(0).call(2, 3) });
    (unsafe { accumulate.spec::<fn(u32, u32)>(1).call(4_u32, 5_u32) });
    assert!(((total) == (26)));
    let mut sub: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: i32, mut y: i32| -> i32 {
            return ((x) - (y));
        },
        |mut x: f64, mut y: f64| -> f64 {
            return ((x) - (y));
        }
    );
    assert!(((unsafe { sub.spec::<fn(i32, i32) -> i32>(0).call(9, 4,) }) == (5)));
    assert!(((unsafe { sub.spec::<fn(f64, f64) -> f64>(1).call(2.5_f64, 1_f64,) }) == (1.5_f64)));
    let mut mixed: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let base: i32 = base;
        },
        |mut x: i32, mut y: i32| -> i32 {
            return (((x) * (y)) + (base));
        },
        |mut x: i32, mut y: f64| -> f64 {
            return (((x as f64) * (y)) + (base as f64));
        }
    );
    assert!(((unsafe { mixed.spec::<fn(i32, i32) -> i32>(0).call(2, 3,) }) == (16)));
    assert!(((unsafe { mixed.spec::<fn(i32, f64) -> f64>(1).call(2, 0.5_f64,) }) == (11_f64)));
    let mut unused: FnPtr<Generic> = lambda_unsafe!(Generic, {});
    &(unused);
    let mut outer: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let base: i32 = base;
        },
        |mut y: i32| -> i32 {
            let mut inner: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut z: i32| -> i32 {
                return ((z) + (1));
            });
            return ((unsafe { inner.spec::<fn(i32) -> i32>(0).call(y) }) + (base));
        }
    );
    assert!(((unsafe { outer.call(5,) }) == (16)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
