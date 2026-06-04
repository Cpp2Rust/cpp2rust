extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn retry_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let count: Value<i32> = <Value<i32>>::default();
    let acc: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *count.borrow_mut() = 0;
            *acc.borrow_mut() = 0;
        }
        'again: {
            (*count.borrow_mut()) += 1;
            (*acc.borrow_mut()) += (*n.borrow());
            if ((((*count.borrow()) < 3) as i32) != 0) {
                goto!('again);
            }
            return (*acc.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ retry_0(4,) }) == 12) as i32) != 0));
    return 0;
}
