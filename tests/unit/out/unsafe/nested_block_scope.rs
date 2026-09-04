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
    let mut x: i32 = 1;
    {
        let mut x: i32 = 2;
        assert!(((x) == (2)));
        {
            let mut x: i32 = 3;
            assert!(((x) == (3)));
        }
        assert!(((x) == (2)));
    }
    assert!(((x) == (1)));
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        let mut y: i32 = i;
        {
            let mut y: i32 = 10;
            sum += y;
        }
        sum += y;
        i.postfix_inc();
    }
    assert!(((sum) == (33)));
    if ((x) == (1)) {
        let mut x: i32 = 5;
        {
            let mut x: i32 = 6;
            assert!(((x) == (6)));
        }
        assert!(((x) == (5)));
    }
    assert!(((x) == (1)));
    return 0;
}
