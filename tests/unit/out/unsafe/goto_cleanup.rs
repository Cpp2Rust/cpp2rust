extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn early_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if ((((n) < (0)) as i32) != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = 100;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_loop_1(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            let mut i: i32 = 0;
            'loop_: while ((((i) < (n)) as i32) != 0) {
                if ((((i) == (3)) as i32) != 0) {
                    ret = 7;
                    goto!('out);
                }
                ret += i;
                i.postfix_inc();
            }
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_switch_2(mut n: i32) -> i32 {
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
                    _ => {
                        ret = 20;
                        break 'switch;
                    }
                }
            };
            ret = 999;
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
            let _n: i32 = -1_i32;
            early_0(_n)
        }) == (-1_i32)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 5;
            early_0(_n)
        }) == (100)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 2;
            from_loop_1(_n)
        }) == (999)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 10;
            from_loop_1(_n)
        }) == (7)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 1;
            from_switch_2(_n)
        }) == (10)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 2;
            from_switch_2(_n)
        }) == (999)) as i32)
            != 0)
    );
    return 0;
}
