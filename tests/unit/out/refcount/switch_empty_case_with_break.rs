extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn empty_case_with_break_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(5));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 9;
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
            let _x: i32 = 1;
            empty_case_with_break_0(_x)
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 2;
            empty_case_with_break_0(_x)
        }) == 2)
    );
    assert!(
        (({
            let _x: i32 = 9;
            empty_case_with_break_0(_x)
        }) == 9)
    );
    return 0;
}
