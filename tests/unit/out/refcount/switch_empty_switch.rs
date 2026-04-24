extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn empty_switch_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            _ => {}
        }
    };
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 5;
            empty_switch_0(_x)
        }) == 5)
    );
    return 0;
}
