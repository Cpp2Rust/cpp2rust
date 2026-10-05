extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_in_switch_continue_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { n } {
            __v if __v == 0 => {
                let mut i: i32 = 0;
                'loop_: while (i < 5) {
                    if ((i % 2) == 0) {
                        i.prefix_inc();
                        continue 'loop_;
                    }
                    r += i;
                    i.prefix_inc();
                }
                break 'switch;
            }
            _ => {
                r = -1_i32;
                break 'switch;
            }
        }
    };
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ for_in_switch_continue_0(0,) }) == 4));
    assert!((({ for_in_switch_continue_0(99,) }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
