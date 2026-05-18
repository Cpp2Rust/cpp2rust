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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let words: Value<Ptr<u16>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u16>(),
    ));
    assert!(((((*words.borrow()).offset((0) as isize).read()) as i32) == 513));
    assert!(((((*words.borrow()).offset((1) as isize).read()) as i32) == 1027));
    (*words.borrow()).offset((0) as isize).write(48042_u16);
    assert!((((*arr.borrow())[(0) as usize] as i32) == 170));
    assert!((((*arr.borrow())[(1) as usize] as i32) == 187));
    assert!((((*arr.borrow())[(2) as usize] as i32) == 3));
    assert!((((*arr.borrow())[(3) as usize] as i32) == 4));
    (*words.borrow_mut()) = (arr.as_pointer() as Ptr<u8>)
        .offset((1) as isize)
        .reinterpret_cast::<u16>();
    assert!(((((*words.borrow()).offset((0) as isize).read()) as i32) == 955));
    (*words.borrow()).offset((0) as isize).write(0_u16);
    assert!((((*arr.borrow())[(0) as usize] as i32) == 170));
    assert!((((*arr.borrow())[(1) as usize] as i32) == 0));
    assert!((((*arr.borrow())[(2) as usize] as i32) == 0));
    assert!((((*arr.borrow())[(3) as usize] as i32) == 4));
    return 0;
}
