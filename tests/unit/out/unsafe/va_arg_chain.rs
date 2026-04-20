extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn extract_nth_0(mut n: i32, mut ap: VaList) -> i32 {
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        ap.arg::<i32>();
        i.postfix_inc();
    }
    return ap.arg::<i32>();
}
pub unsafe fn middle_layer_1(mut n: i32, mut ap: VaList) -> i32 {
    return (unsafe {
        let _n: i32 = n;
        let _ap: VaList = ap;
        extract_nth_0(_n, _ap)
    });
}
pub unsafe fn top_level_2(mut n: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(args);
    let mut result: i32 = (unsafe {
        let _n: i32 = n;
        let _ap: VaList = ap;
        middle_layer_1(_n, _ap)
    });
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _n: i32 = 2;
            top_level_2(_n, &[100.into(), 200.into(), 300.into(), 400.into()])
        }) == (300))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 0;
            top_level_2(_n, &[42.into(), 99.into()])
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 3;
            top_level_2(_n, &[1.into(), 2.into(), 3.into(), 4.into()])
        }) == (4))
    );
    return 0;
}
