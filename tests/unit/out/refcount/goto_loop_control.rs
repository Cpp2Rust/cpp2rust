extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn loopctl_0() -> i32 {
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 5) as i32) != 0) {
        goto_block!({
            '__entry: {
                if ((((*i.borrow()) == 1) as i32) != 0) {
                    (*i.borrow_mut()).postfix_inc();
                    continue 'loop_;
                }
                if ((((*i.borrow()) == 4) as i32) != 0) {
                    break;
                }
                goto!('add);
            }
            'add: {
                (*sum.borrow_mut()) += 1;
            }
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return (*sum.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ loopctl_0() }) == 3) as i32) != 0));
    return 0;
}
