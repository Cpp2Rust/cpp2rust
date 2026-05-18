extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn mixed_args_0(mut count: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((((i) < (count)) as i32) != 0) {
        let mut tag: i32 = ap.arg::<i32>();
        if ((((tag) == (0)) as i32) != 0) {
            total += ap.arg::<i32>();
        } else {
            let mut ptr: *mut i32 = ap.arg::<*mut i32>();
            total += (*ptr);
        }
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 100;
    assert!(
        ((((unsafe {
            let _count: i32 = 3;
            mixed_args_0(
                _count,
                &[
                    0.into(),
                    10.into(),
                    1.into(),
                    (&mut x as *mut i32).into(),
                    0.into(),
                    20.into(),
                ],
            )
        }) == (130)) as i32)
            != 0)
    );
    let mut y: i32 = 50;
    assert!(
        ((((unsafe {
            let _count: i32 = 1;
            mixed_args_0(_count, &[1.into(), (&mut y as *mut i32).into()])
        }) == (50)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _count: i32 = 2;
            mixed_args_0(_count, &[0.into(), 5.into(), 0.into(), 3.into()])
        }) == (8)) as i32)
            != 0)
    );
    return 0;
}
