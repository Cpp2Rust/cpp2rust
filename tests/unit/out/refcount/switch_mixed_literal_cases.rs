extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_literal_cases_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == -1_i32 => {
                return 1;
            }
            __v if __v == 16 => {
                return 2;
            }
            __v if __v == 65152 => {
                return 3;
            }
            __v if __v == -255_i32 => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ mixed_literal_cases_0(-1_i32,) }) == 1));
    assert!((({ mixed_literal_cases_0(16,) }) == 2));
    assert!((({ mixed_literal_cases_0(65152,) }) == 3));
    assert!((({ mixed_literal_cases_0(-255_i32,) }) == 4));
    assert!((({ mixed_literal_cases_0(7,) }) == 0));
    return 0;
}
