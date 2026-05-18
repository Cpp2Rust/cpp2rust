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
    let mut out: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        if (((i) % (2)) == (0)) {
            i.prefix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        i.prefix_inc();
    }
    let mut j: i32 = 0;
    'loop_: while ((j) < (10)) {
        if (((i) % (2)) == (0)) {
            j.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        j.postfix_inc();
    }
    let mut k1: i32 = 0;
    'loop_: while ((k1) < (5)) {
        let mut k2: i32 = 0;
        'loop_: while ((k2) < (5)) {
            let mut k3: i32 = 0;
            'loop_: while ((k3) < (5)) {
                if (((((k1) + (k2)) + (k3)) % (2)) == (0)) {
                    k3.postfix_inc();
                    continue 'loop_;
                }
                out.prefix_inc();
                k3.postfix_inc();
            }
            if ((((k1) + (k2)) % (2)) == (0)) {
                k2.postfix_inc();
                continue 'loop_;
            }
            out.prefix_inc();
            k2.postfix_inc();
        }
        if (((k1) % (2)) == (0)) {
            k1.postfix_inc();
            continue 'loop_;
        }
        out.prefix_inc();
        k1.postfix_inc();
    }
    return out;
}
