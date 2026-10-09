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
    let mut v: Vec<i32> = vec![1, 2];
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let v: Vec<i32> = v.clone();
        },
        || -> i32 {
            return (v.len() as i32);
        },
    );
    let mut g: FnPtr<fn() -> i32> = f.copy_from();
    let mut h: FnPtr<fn() -> i32> = f.move_from();
    assert!(((unsafe { g.call() }) == (2)));
    assert!(((unsafe { h.call() }) == (2)));
    assert!(((unsafe { f.call() }) == (0)));
    let mut p: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let u: Option<Box<i32>> = Some(Box::new(5));
        },
        || -> i32 {
            return if !(u
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            .is_null()
            {
                (*(*(std::ptr::addr_of!(u).cast_mut()))
                    .as_deref_mut()
                    .unwrap())
            } else {
                0
            };
        },
    );
    let mut q: FnPtr<fn() -> i32> = p.move_from();
    assert!(((unsafe { q.call() }) == (5)));
    assert!(((unsafe { p.call() }) == (0)));
    let mut n: i32 = 0;
    let mut inner: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let n: i32 = n;
        },
        || -> i32 {
            return n.prefix_inc();
        },
    );
    let mut outer: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let inner: FnPtr<fn() -> i32> = inner.copy_from();
        },
        || -> i32 {
            return (unsafe { inner.call() });
        },
    );
    assert!(((unsafe { outer.call() }) == (1)));
    let mut outer2: FnPtr<fn() -> i32> = outer.copy_from();
    assert!(((unsafe { outer2.call() }) == (2)));
    assert!(((unsafe { outer.call() }) == (2)));
    let mut outer3: FnPtr<fn() -> i32> = outer.move_from();
    assert!(((unsafe { outer3.call() }) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
