extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_return_break_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 0 => {
                return 100;
            }
            v if v == 1 => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
            v if v == 2 => {
                return 200;
            }
            _ => {
                (*r.borrow_mut()) = 99;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 0;
            mixed_return_break_0(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 1;
            mixed_return_break_0(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 2;
            mixed_return_break_0(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 99;
            mixed_return_break_0(_x)
        }) == 99)
    );
    return 0;
}
