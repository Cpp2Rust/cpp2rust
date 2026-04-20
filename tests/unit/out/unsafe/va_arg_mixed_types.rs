extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_mixed_0(mut count: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(args);
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (count)) {
        let mut tag: i32 = ap.arg::<i32>();
        if ((tag) == (0)) {
            total += ap.arg::<i32>();
        } else if ((tag) == (1)) {
            total += (ap.arg::<f64>() as i32);
        } else {
            let mut val: i64 = ap.arg::<i64>();
            total += (val as i32);
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
    assert!(
        ((unsafe {
            let _count: i32 = 3;
            sum_mixed_0(
                _count,
                &[
                    0.into(),
                    10.into(),
                    1.into(),
                    2.05E+1.into(),
                    2.into(),
                    30_i64.into(),
                ],
            )
        }) == (60))
    );
    assert!(
        ((unsafe {
            let _count: i32 = 1;
            sum_mixed_0(_count, &[0.into(), 42.into()])
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _count: i32 = 2;
            sum_mixed_0(_count, &[1.into(), 3.7E+0.into(), 2.into(), 100_i64.into()])
        }) == (103))
    );
    return 0;
}
