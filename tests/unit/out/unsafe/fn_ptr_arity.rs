extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(
    mut a1: i32,
    mut a2: i32,
    mut a3: i32,
    mut a4: i32,
    mut a5: i32,
    mut a6: i32,
    mut a7: i32,
    mut a8: i32,
    mut a9: i32,
    mut a10: i32,
    mut a11: i32,
    mut a12: i32,
    mut a13: i32,
    mut a14: i32,
) -> i32 {
    return 22;
}
pub unsafe fn wide_1(
    mut a1: i32,
    mut a2: i32,
    mut a3: i32,
    mut a4: i32,
    mut a5: i32,
    mut a6: i32,
    mut a7: i32,
    mut a8: i32,
    mut a9: i32,
    mut a10: i32,
    mut a11: i32,
    mut a12: i32,
    mut a13: i32,
    mut a14: i32,
    mut a15: i32,
    mut a16: i32,
    mut a17: i32,
    mut a18: i32,
    mut a19: i32,
    mut a20: i32,
    mut a21: i32,
    mut a22: i32,
) -> i32 {
    return ((a1) + (a22));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Option<
        unsafe fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
    > = (Some(foo_0));
    assert!(((unsafe { (f).unwrap()(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,) }) == (22)));
    let mut w: Option<
        unsafe fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) -> i32,
    > = (Some(wide_1));
    assert!(
        ((unsafe {
            (w).unwrap()(
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            )
        }) == (23))
    );
    let mut l: FnPtr<
        fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) -> i32,
    > = FnPtr::<
        fn(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) -> i32,
    >::new(
        |a1: i32,
         a2: i32,
         a3: i32,
         a4: i32,
         a5: i32,
         a6: i32,
         a7: i32,
         a8: i32,
         a9: i32,
         a10: i32,
         a11: i32,
         a12: i32,
         a13: i32,
         a14: i32,
         a15: i32,
         a16: i32,
         a17: i32,
         a18: i32|
         -> i32 {
            unsafe {
                return ((a1) * (a18));
            }
        },
    );
    assert!(((unsafe { l.call(2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,) }) == (18)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
