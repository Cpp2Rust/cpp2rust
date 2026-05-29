extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn skip_0(mut n: i32) -> i32 {
    let mut x: i32 = 0_i32;
    goto_block!({
        '__entry: {
            x = 0;
            if ((((n) > (0)) as i32) != 0) {
                goto!('mid);
            }
            x += 10;
        }
        'mid: {
            x += 1;
            return x;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _n: i32 = 1;
            skip_0(_n)
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = -1_i32;
            skip_0(_n)
        }) == (11)) as i32)
            != 0)
    );
    return 0;
}
