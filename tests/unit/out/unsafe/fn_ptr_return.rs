extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut x: i32) -> i32 {
    return ((x) + (1));
}
pub unsafe fn dec_1(mut x: i32) -> i32 {
    return ((x) - (1));
}
pub unsafe fn pick_2(mut choose_inc: i32) -> Option<unsafe fn(i32) -> i32> {
    if (choose_inc != 0) {
        return Some(inc_0);
    }
    return Some(dec_1);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Option<unsafe fn(i32) -> i32> = (unsafe {
        let _choose_inc: i32 = 1;
        pick_2(_choose_inc)
    });
    assert!(!((f).is_none()));
    assert!(((f) == (Some(inc_0))));
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            (f).unwrap()(_arg0)
        }) == (11))
    );
    let mut g: Option<unsafe fn(i32) -> i32> = (unsafe {
        let _choose_inc: i32 = 0;
        pick_2(_choose_inc)
    });
    assert!(((g) == (Some(dec_1))));
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            (g).unwrap()(_arg0)
        }) == (9))
    );
    assert!(((f) != (g)));
    return 0;
}
