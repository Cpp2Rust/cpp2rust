extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_switch_for_break_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        'switch: {
            let __match_cond = (*i.borrow());
            match __match_cond {
                v if v == 1 => {
                    let j: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*j.borrow()) < 10) {
                        if ((*j.borrow()) == 2) {
                            break 'switch;
                        }
                        (*r.borrow_mut()) += 1;
                        (*j.borrow_mut()).prefix_inc();
                    }
                    (*r.borrow_mut()) += 100;
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 10;
                    break 'switch;
                }
            }
        };
        (*i.borrow_mut()).prefix_inc();
    }
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _n: i32 = 3;
            for_switch_for_break_0(_n)
        }) == 122)
    );
    return 0;
}
