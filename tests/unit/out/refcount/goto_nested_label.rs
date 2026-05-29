extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn scan_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*n.borrow())) as i32) != 0) {
        let mut j: Value<i32> = <Value<i32>>::default();
        goto_block!({
            '__entry: {
                j = Rc::new(RefCell::new(0));
                'loop_: while ((((*j.borrow()) < 10) as i32) != 0) {
                    if ((((*j.borrow()) == 5) as i32) != 0) {
                        goto!('next);
                    }
                    (*total.borrow_mut()) += 1;
                    (*j.borrow_mut()).postfix_inc();
                }
                (*total.borrow_mut()) += 100;
            }
            'next: {
                (*total.borrow_mut()) += 1000;
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
            let _n: i32 = 2;
            scan_0(_n)
        }) == 2010) as i32)
            != 0)
    );
    return 0;
}
