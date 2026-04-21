extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0_i32;
    if ({
        b = a;
        b
    } != 0)
    {}
    'loop_: while ((({
        b = a;
        b
    }) as i32)
        != (0))
    {}
    if (a != 0) {}
    if ((a) == (b)) {}
    if ((a) < (b)) {}
    assert!(((a) == (b)));
    assert!(
        (!(({
            a = b;
            a
        }) as i32)
            != 0)
    );
    let mut c: bool = false;
    c = ({
        a = b;
        a
    } != 0);
    c = ((({
        b = a;
        b
    }) as i32)
        != (0));
    c = (a != 0);
    c = ((a) == (b));
    c = ((a) < (b));
    return 0;
}
