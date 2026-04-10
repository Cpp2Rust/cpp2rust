extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_ints_0(mut first: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = <VaList>::default();
    let mut total: i32 = first;
    ap = VaList::new(args);
    let mut val: i32 = <i32>::default();
    'loop_: while ((({
        val = ap.arg::<i32>();
        val
    }) as i32)
        != (0))
    {
        total += val;
    }
    return total;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return ((unsafe {
        let _first: i32 = 1;
        sum_ints_0(_first, &[2.into(), 3.into(), 4.into(), 0.into()])
    }) - (10));
}
