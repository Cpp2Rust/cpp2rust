extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        1125912791875585_u64,
        2251829878849541_u64,
    ])));
    let words: Value<Ptr<u16>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u64>).reinterpret_cast::<u16>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 8) {
        assert!({
            let _lhs = (((*words.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == ((((*i.borrow()) + 1) as u16) as i32)
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return 0;
}
