extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn run_0() -> i32 {
    let mut i: i32 = 0;
    let mut runs: i32 = 0;
    let mut __do_while = true;
    'loop_: while __do_while || ((((i) < (4)) as i32) != 0) {
        __do_while = false;
        runs += 1;
        i += 1;
        if ((((i) == (4)) as i32) != 0) {
            continue 'loop_;
        }
    }
    return runs;
}
pub unsafe fn nested_1() -> i32 {
    let mut oi: i32 = 0;
    let mut runs: i32 = 0;
    let mut __do_while = true;
    'loop_: while __do_while || ((((oi) < (2)) as i32) != 0) {
        __do_while = false;
        oi += 1;
        let mut ii: i32 = 0;
        let mut __do_while = true;
        'loop_: while __do_while || ((((ii) < (3)) as i32) != 0) {
            __do_while = false;
            runs += 1;
            ii += 1;
            if ((((ii) == (3)) as i32) != 0) {
                continue 'loop_;
            }
        }
    }
    return runs;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { run_0() }) == (4)) as i32) != 0));
    assert!(((((unsafe { nested_1() }) == (6)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
