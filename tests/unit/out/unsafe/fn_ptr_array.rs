extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_0(mut a: i32, mut b: i32) -> i32 {
    return ((a) + (b));
}
pub unsafe fn sub_1(mut a: i32, mut b: i32) -> i32 {
    return ((a) - (b));
}
pub unsafe fn mul_2(mut a: i32, mut b: i32) -> i32 {
    return ((a) * (b));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ops: [Option<unsafe fn(i32, i32) -> i32>; 3] =
        [Some(add_0 as _), Some(sub_1 as _), Some(mul_2 as _)];
    assert!(
        ((unsafe {
            let _arg0: i32 = 2;
            let _arg1: i32 = 3;
            (ops[(0) as usize]).unwrap()(_arg0, _arg1)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _arg0: i32 = 7;
            let _arg1: i32 = 4;
            (ops[(1) as usize]).unwrap()(_arg0, _arg1)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _arg0: i32 = 6;
            let _arg1: i32 = 5;
            (ops[(2) as usize]).unwrap()(_arg0, _arg1)
        }) == (30))
    );
    assert!(!((ops[(0) as usize]).is_none()));
    assert!(((ops[(0) as usize]) == (Some(add_0 as _))));
    assert!(((ops[(0) as usize]) != (Some(sub_1 as _))));
    return 0;
}
