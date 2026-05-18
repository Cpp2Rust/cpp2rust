extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_in_switch_continue_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*n.borrow());
        match __match_cond {
            v if v == 0 => {
                let i: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*i.borrow()) < 5) {
                    if (((*i.borrow()) % 2) == 0) {
                        (*i.borrow_mut()).prefix_inc();
                        continue 'loop_;
                    }
                    (*r.borrow_mut()) += (*i.borrow());
                    (*i.borrow_mut()).prefix_inc();
                }
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = -1_i32;
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
            let _n: i32 = 0;
            for_in_switch_continue_0(_n)
        }) == 4)
    );
    assert!(
        (({
            let _n: i32 = 99;
            for_in_switch_continue_0(_n)
        }) == -1_i32)
    );
    return 0;
}
