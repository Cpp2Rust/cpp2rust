extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_then_product_0(mut first: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    let mut sum: i32 = first;
    let mut product: i32 = first;
    ap = VaList::new(args);
    let mut val: i32 = 0_i32;
    'loop_: while ((({
        val = ap.arg::<i32>();
        val
    }) as i32)
        != (0))
    {
        sum += val;
    }
    ap = VaList::new(args);
    'loop_: while ((({
        val = ap.arg::<i32>();
        val
    }) as i32)
        != (0))
    {
        product *= val;
    }
    return ((sum) + (product));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _first: i32 = 2;
            sum_then_product_0(_first, &[3.into(), 4.into(), 0.into()])
        }) == (33))
    );
    return 0;
}
