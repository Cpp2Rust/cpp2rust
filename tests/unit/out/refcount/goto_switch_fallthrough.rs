extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            switch!(match (*n.borrow()) {
                __v if __v == 0 => {
                    (*ret.borrow_mut()) += 1;
                }
                __v if __v == 1 => {
                    (*ret.borrow_mut()) += 10;
                    goto!('out);
                }
                _ => {
                    (*ret.borrow_mut()) += 100;
                    break;
                }
            });
            (*ret.borrow_mut()) += 1000;
        }
        'out: {
            return (*ret.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ sm_0(0,) }) == 11) as i32) != 0));
    assert!((((({ sm_0(1,) }) == 10) as i32) != 0));
    assert!((((({ sm_0(9,) }) == 1100) as i32) != 0));
    return 0;
}
