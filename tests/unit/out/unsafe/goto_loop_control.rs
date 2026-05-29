extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn loopctl_0(mut n: i32) -> i32 {
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        goto_block!({
            '__entry: {
                if ((((i) == (2)) as i32) != 0) {
                    i.postfix_inc();
                    continue 'loop_;
                }
                if ((((i) == (5)) as i32) != 0) {
                    break;
                }
                if (((((i) % (2)) == (0)) as i32) != 0) {
                    goto!('even);
                }
                total += 1;
            }
            'even: {
                total += 10;
            }
        });
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _n: i32 = 10;
            loopctl_0(_n)
        }) == (42)) as i32)
            != 0)
    );
    return 0;
}
