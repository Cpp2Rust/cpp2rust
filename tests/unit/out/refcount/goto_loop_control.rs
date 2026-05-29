extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn loopctl_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*n.borrow())) as i32) != 0) {
        goto_block!({
            '__entry: {
                if ((((*i.borrow()) == 2) as i32) != 0) {
                    (*i.borrow_mut()).postfix_inc();
                    continue 'loop_;
                }
                if ((((*i.borrow()) == 5) as i32) != 0) {
                    break;
                }
                if (((((*i.borrow()) % 2) == 0) as i32) != 0) {
                    goto!('even);
                }
                (*total.borrow_mut()) += 1;
            }
            'even: {
                (*total.borrow_mut()) += 10;
            }
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return (*total.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _n: i32 = 10;
            loopctl_0(_n)
        }) == 42) as i32)
            != 0)
    );
    return 0;
}
