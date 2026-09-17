extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn run_0() -> i32 {
    let i: Value<i32> = Rc::new(RefCell::new(0));
    let runs: Value<i32> = Rc::new(RefCell::new(0));
    let mut __do_while = true;
    'loop_: while __do_while || ((((*i.borrow()) < 4) as i32) != 0) {
        __do_while = false;
        (*runs.borrow_mut()) += 1;
        (*i.borrow_mut()) += 1;
        if ((((*i.borrow()) == 4) as i32) != 0) {
            continue 'loop_;
        }
    }
    return (*runs.borrow());
}
pub fn nested_1() -> i32 {
    let oi: Value<i32> = Rc::new(RefCell::new(0));
    let runs: Value<i32> = Rc::new(RefCell::new(0));
    let mut __do_while = true;
    'loop_: while __do_while || ((((*oi.borrow()) < 2) as i32) != 0) {
        __do_while = false;
        (*oi.borrow_mut()) += 1;
        let ii: Value<i32> = Rc::new(RefCell::new(0));
        let mut __do_while = true;
        'loop_: while __do_while || ((((*ii.borrow()) < 3) as i32) != 0) {
            __do_while = false;
            (*runs.borrow_mut()) += 1;
            (*ii.borrow_mut()) += 1;
            if ((((*ii.borrow()) == 3) as i32) != 0) {
                continue 'loop_;
            }
        }
    }
    return (*runs.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ run_0() }) == 4) as i32) != 0));
    assert!((((({ nested_1() }) == 6) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
