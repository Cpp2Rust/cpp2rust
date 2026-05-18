extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_on_assignment_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = {
            (*y.borrow_mut()) = ((*x.borrow()) + 1);
            (*y.borrow())
        };
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 20;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = (*y.borrow());
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
            switch_on_assignment_0(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 1;
            switch_on_assignment_0(_x)
        }) == 20)
    );
    assert!(
        (({
            let _x: i32 = 9;
            switch_on_assignment_0(_x)
        }) == 10)
    );
    return 0;
}
