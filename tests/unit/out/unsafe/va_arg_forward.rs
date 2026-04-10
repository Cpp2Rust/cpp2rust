extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inner_0(mut count: i32, mut ap: VaList) -> i32 {
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (count)) {
        total += ap.arg::<i32>();
        i.postfix_inc();
    }
    return total;
}
pub unsafe fn outer_1(mut count: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = <VaList>::default();
    ap = VaList::new(args);
    let mut result: i32 = (unsafe {
        let _count: i32 = count;
        let _ap: VaList = ap;
        inner_0(_count, _ap)
    });
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return ((unsafe {
        let _count: i32 = 3;
        outer_1(_count, &[10.into(), 20.into(), 30.into()])
    }) - (60));
}
