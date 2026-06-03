extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn loopctl_0() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((((i) < (5)) as i32) != 0) {
        goto_block!({
            '__entry: {
                if ((((i) == (1)) as i32) != 0) {
                    i.postfix_inc();
                    continue 'loop_;
                }
                if ((((i) == (4)) as i32) != 0) {
                    break;
                }
                goto!('add);
            }
            'add: {
                sum += 1;
            }
        });
        i.postfix_inc();
    }
    return sum;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { loopctl_0() }) == (3)) as i32) != 0));
    return 0;
}
