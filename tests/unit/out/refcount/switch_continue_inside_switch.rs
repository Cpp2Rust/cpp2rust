extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn continue_inside_switch_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        switch!(match (*i.borrow()) {
            v if v == 0 || v == 2 || v == 4 => {
                (*i.borrow_mut()).prefix_inc();
                continue 'loop_;
            }
            _ => {
                (*r.borrow_mut()) += (*i.borrow());
                break;
            }
        });
        (*r.borrow_mut()) += 1000;
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
            let _n: i32 = 6;
            continue_inside_switch_0(_n)
        }) == ((((1 + 3) + 5) as i32) + (3 * 1000)))
    );
    return 0;
}
