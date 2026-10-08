extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn nested_0(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { a } {
            __v if __v == 1 => {
                'switch: {
                    match { b } {
                        __v if __v == 10 => {
                            r = 11;
                            break 'switch;
                        }
                        __v if __v == 20 => {
                            r = 12;
                            break 'switch;
                        }
                        _ => {
                            r = 13;
                            break 'switch;
                        }
                    }
                };
                r += 1;
                break 'switch;
            }
            __v if __v == 2 => {
                r = 2;
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
    assert!((({ nested_0(1, 10,) }) == 12));
    assert!((({ nested_0(1, 99,) }) == 14));
    assert!((({ nested_0(2, 0,) }) == 2));
    assert!((({ nested_0(3, 3,) }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
