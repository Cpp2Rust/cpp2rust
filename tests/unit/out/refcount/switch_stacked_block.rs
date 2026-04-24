extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn stacked_block_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                let y: Value<i32> = Rc::new(RefCell::new(((*x.borrow()) * 2)));
                (*r.borrow_mut()) = ((*y.borrow()) + 1);
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 0;
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
            let _x: i32 = 2;
            stacked_block_0(_x)
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 9;
            stacked_block_0(_x)
        }) == 0)
    );
    return 0;
}
