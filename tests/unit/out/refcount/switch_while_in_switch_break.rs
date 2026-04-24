extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn while_in_switch_break_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*n.borrow()) {
        v if v == 0 => {
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < 10) {
                if ((*i.borrow()) == 4) {
                    break;
                }
                (*r.borrow_mut()) += (*i.borrow());
                (*i.borrow_mut()).prefix_inc();
            }
            (*r.borrow_mut()) += 1000;
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _n: i32 = 0;
            while_in_switch_break_0(_n)
        }) == 1006)
    );
    assert!(
        (({
            let _n: i32 = 99;
            while_in_switch_break_0(_n)
        }) == -1_i32)
    );
    return 0;
}
