extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_ints_0(mut first: i32, __args: &[VaArg]) -> i32 {
    let mut args: VaList = VaList::default();
    let mut total: i32 = first;
    args = VaList::new(__args);
    let mut val: i32 = 0_i32;
    'loop_: while (((({
        val = args.arg::<i32>();
        val
    }) != (0)) as i32)
        != 0)
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
    assert!(
        ((((unsafe {
            let _first: i32 = 1;
            sum_ints_0(_first, &[2.into(), 3.into(), 4.into(), 0.into()])
        }) == (10)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _first: i32 = 100;
            sum_ints_0(_first, &[0.into()])
        }) == (100)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _first: i32 = 5;
            sum_ints_0(_first, &[5.into(), 5.into(), 5.into(), 5.into(), 0.into()])
        }) == (25)) as i32)
            != 0)
    );
    return 0;
}
