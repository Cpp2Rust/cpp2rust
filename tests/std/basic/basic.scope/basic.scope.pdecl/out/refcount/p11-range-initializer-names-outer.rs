extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 5, 9])));
    let mut steps: i32 = 0;
    let mut sum: i32 = 0;
    'loop_: for values in IterRange::new(values.decay(), values.decay().offset((3_i64) as isize)) {
        let mut values: i32 = (values.read());
        values *= 2;
        steps.prefix_inc();
        sum += values;
    }
    assert!((steps == 3));
    assert!((sum == 32));
    assert!(({ ((*values.borrow())[(0) as usize] == 2) }));
    assert!(({ ((*values.borrow())[(1) as usize] == 5) }));
    assert!(({ ((*values.borrow())[(2) as usize] == 9) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
