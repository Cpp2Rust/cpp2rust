extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            'switch: {
                let __match_cond = n;
                match __match_cond {
                    v if v == 1 => {
                        ret = 10;
                        goto!('out);
                    }
                    v if v == 2 => {
                        ret = 20;
                        break 'switch;
                    }
                    _ => {
                        ret = 30;
                        break 'switch;
                    }
                }
            };
            ret += 1;
        }
        'out: {
            return ret;
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
            classify_0(_n)
        }) == (10)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 2;
            classify_0(_n)
        }) == (21)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 9;
            classify_0(_n)
        }) == (31)) as i32)
            != 0)
    );
    return 0;
}
