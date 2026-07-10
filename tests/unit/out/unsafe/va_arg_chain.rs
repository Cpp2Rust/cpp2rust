extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn extract_nth_0(mut n: i32, mut ap: VaList) -> i32 {
    let mut i: i32 = 0;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        ap.arg::<i32>();
        i.postfix_inc();
    }
    return ap.arg::<i32>();
}
pub unsafe fn middle_layer_1(mut n: i32, mut ap: VaList) -> i32 {
    return (unsafe { extract_nth_0(n, ap) });
}
pub unsafe fn top_level_2(mut n: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut result: i32 = (unsafe { middle_layer_1(n, ap) });
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            top_level_2(2, &[(100).into(), (200).into(), (300).into(), (400).into()])
        }) == (300)) as i32)
            != 0)
    );
    assert!(((((unsafe { top_level_2(0, &[(42).into(), (99).into(),]) }) == (42)) as i32) != 0));
    assert!(
        ((((unsafe { top_level_2(3, &[(1).into(), (2).into(), (3).into(), (4).into(),]) }) == (4))
            as i32)
            != 0)
    );
    return 0;
}
