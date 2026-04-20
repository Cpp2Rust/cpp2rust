extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_with_copy_0(mut count: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    let mut aq: VaList = VaList::default();
    ap = VaList::new(args);
    aq = ap.clone();
    let mut sum1: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (count)) {
        sum1 += ap.arg::<i32>();
        i.postfix_inc();
    }
    let mut sum2: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (count)) {
        sum2 += aq.arg::<i32>();
        i.postfix_inc();
    }
    assert!(((sum1) == (sum2)));
    return ((sum1) + (sum2));
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
            sum_with_copy_0(_count, &[10.into(), 20.into(), 30.into()])
        }) == (120))
    );
    return 0;
}
