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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut values: [i32; 3] = [2, 5, 9];
    let mut steps: i32 = 0;
    let mut sum: i32 = 0;
    'loop_: for values in IterRange::new(
        values.as_mut_ptr(),
        values.as_mut_ptr().offset((3_i64) as isize),
    ) {
        let mut values: i32 = (*values);
        values *= 2;
        steps.prefix_inc();
        sum += values;
    }
    assert!(((steps) == (3)));
    assert!(((sum) == (32)));
    assert!(((values[(0) as usize]) == (2)));
    assert!(((values[(1) as usize]) == (5)));
    assert!(((values[(2) as usize]) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
