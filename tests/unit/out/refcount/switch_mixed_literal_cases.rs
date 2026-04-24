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
            v if v == -1_i32 => {
                return 1;
            }
            v if v == 16 => {
                return 2;
            }
            v if v == 65152 => {
                return 3;
            }
            v if v == -255_i32 => {
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
    assert!(
        (({
            let _x: i32 = -1_i32;
            mixed_literal_cases_0(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 16;
            mixed_literal_cases_0(_x)
        }) == 2)
    );
    assert!(
        (({
            let _x: i32 = 65152;
            mixed_literal_cases_0(_x)
        }) == 3)
    );
    assert!(
        (({
            let _x: i32 = -255_i32;
            mixed_literal_cases_0(_x)
        }) == 4)
    );
    assert!(
        (({
            let _x: i32 = 7;
            mixed_literal_cases_0(_x)
        }) == 0)
    );
    return 0;
}
