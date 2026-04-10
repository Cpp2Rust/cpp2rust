extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_mixed_0(mut count: i32, args: &[VaArg]) -> f64 {
    let mut ap: VaList = <VaList>::default();
    ap = VaList::new(args);
    let mut total: f64 = 0_f64;
    let mut i: i32 = 0;
    'loop_: while ((i) < (count)) {
        total += ap.arg::<f64>();
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
    return (((unsafe {
        let _count: i32 = 3;
        sum_mixed_0(_count, &[1.5E+0.into(), 2.5E+0.into(), 3.0E+0.into()])
    }) as i32)
        - (7));
}
