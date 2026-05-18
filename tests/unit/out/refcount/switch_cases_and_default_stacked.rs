extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cases_and_default_stacked_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 3 => {
                (*r.borrow_mut()) = 3;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 42;
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
            cases_and_default_stacked_0(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 2;
            cases_and_default_stacked_0(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 3;
            cases_and_default_stacked_0(_x)
        }) == 3)
    );
    assert!(
        (({
            let _x: i32 = 99;
            cases_and_default_stacked_0(_x)
        }) == 42)
    );
    return 0;
}
