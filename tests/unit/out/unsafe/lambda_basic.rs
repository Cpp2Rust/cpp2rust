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
    let mut zero: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return 42;
        }
    });
    assert!(((unsafe { zero.call() }) == (42)));
    let mut one: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        unsafe {
            return ((x) + (1));
        }
    });
    assert!(((unsafe { one.call(1,) }) == (2)));
    let mut three: FnPtr<fn(i32, i32, i32) -> i32> =
        FnPtr::<fn(i32, i32, i32) -> i32>::new(|mut x: i32, mut y: i32, mut z: i32| -> i32 {
            unsafe {
                return ((((x) * (100)) + ((y) * (10))) + (z));
            }
        });
    assert!(((unsafe { three.call(1, 2, 3,) }) == (123)));
    let k: i32 = 3;
    let m: i32 = 4;
    let mut constants: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        unsafe {
            return (((x) + (3)) + (4));
        }
    });
    assert!(((unsafe { constants.call(1,) }) == (8)));
    let n: i32 = ((k) + (m));
    let mut derived: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        unsafe {
            return ((x) + ((3) + (4)));
        }
    });
    assert!(((unsafe { derived.call(1,) }) == (8)));
    let mut implicit: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        unsafe {
            return ((x) + (3));
        }
    });
    assert!(((unsafe { implicit.call(1,) }) == (4)));
    let mut bump: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        unsafe {
            x += 1;
            return x;
        }
    });
    assert!(((unsafe { bump.call(1,) }) == (2)));
    let mut through_ptr: FnPtr<fn(i32) -> i32> =
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            unsafe {
                let mut p: *mut i32 = (&mut x as *mut i32);
                (*p) = ((*p) * (2));
                return x;
            }
        });
    assert!(((unsafe { through_ptr.call(4,) }) == (8)));
    let mut seed: i32 = 5;
    let mut boxed: i32 = (unsafe { through_ptr.call(seed) });
    let mut boxed_ptr: *mut i32 = (&mut boxed as *mut i32);
    assert!(((*boxed_ptr) == (10)));
    assert!(((seed) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
