extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type color = u32;
pub const color_RED: color = 0;
pub const color_GREEN: color = 1;
pub const color_BLUE: color = 2;
pub const color_COLOR_LAST: color = 3;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut count: i32 = 0;
    let mut c: color = color_RED;
    'loop_: while ((((c as u32) < ((color_COLOR_LAST as i32) as u32)) as i32) != 0) {
        count.postfix_inc();
        c.postfix_inc();
    }
    assert!(((((count) == (3)) as i32) != 0));
    let mut c: color = color_RED;
    assert!(((((c.postfix_inc() as u32) == ((color_RED as i32) as u32)) as i32) != 0));
    assert!(((((c.prefix_inc() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0));
    assert!(((((c.postfix_dec() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0));
    assert!(((((c.prefix_dec() as u32) == ((color_RED as i32) as u32)) as i32) != 0));
    return 0;
}
