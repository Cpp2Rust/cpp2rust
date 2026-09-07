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
    let a: Value<u32> = Rc::new(RefCell::new(12_u32));
    let b: Value<u32> = Rc::new(RefCell::new(10_u32));
    let z: Value<u32> = Rc::new(RefCell::new(0_u32));
    assert!((!((*z.borrow()) != 0)));
    assert!((((*a.borrow()) != 0) && ((*b.borrow()) != 0)));
    assert!((!(((*a.borrow()) != 0) && ((*z.borrow()) != 0))));
    assert!((((*a.borrow()) != 0) || ((*z.borrow()) != 0)));
    assert!(((*a.borrow()) != (*b.borrow())));
    assert!(((!(*a.borrow())) == !12_u32));
    assert!((((*a.borrow()) & (*b.borrow())) == 8_u32));
    assert!((((*a.borrow()) | (*b.borrow())) == 14_u32));
    assert!((((*a.borrow()) ^ (*b.borrow())) == 6_u32));
    (*a.borrow_mut()) &= (*b.borrow());
    assert!(((*a.borrow()) == 8_u32));
    (*a.borrow_mut()) |= (*b.borrow());
    assert!(((*a.borrow()) == 10_u32));
    (*a.borrow_mut()) ^= (*b.borrow());
    assert!(((*a.borrow()) == 0_u32));
    return 0;
}
