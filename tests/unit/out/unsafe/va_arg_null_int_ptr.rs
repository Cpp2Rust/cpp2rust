extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_nonnull_0(mut count: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(args);
    let mut result: i32 = -1_i32;
    let mut i: i32 = 0;
    'loop_: while ((((i) < (count)) as i32) != 0) {
        let mut p: *mut i32 = ap.arg::<*mut i32>();
        if (((!((p).is_null())) as i32) != 0) {
            result = (*p);
            break;
        }
        i.postfix_inc();
    }
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 42;
    assert!(
        ((((unsafe {
            let _count: i32 = 2;
            first_nonnull_0(
                _count,
                &[
                    (0 as *mut ::libc::c_void).into(),
                    (&mut x as *mut i32).into(),
                ],
            )
        }) == (42)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _count: i32 = 3;
            first_nonnull_0(
                _count,
                &[
                    (0 as *mut ::libc::c_void).into(),
                    (0 as *mut ::libc::c_void).into(),
                    (&mut x as *mut i32).into(),
                ],
            )
        }) == (42)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _count: i32 = 1;
            first_nonnull_0(_count, &[(0 as *mut ::libc::c_void).into()])
        }) == (-1_i32)) as i32)
            != 0)
    );
    return 0;
}
