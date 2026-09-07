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
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8])));
    {
        ((buf.as_pointer() as Ptr<u8>).offset((2) as isize) as Ptr<u8>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                4_usize as usize,
            );
        ((buf.as_pointer() as Ptr<u8>).offset((2) as isize) as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!((((*buf.borrow())[(0) as usize] as i32) == 1));
    assert!((((*buf.borrow())[(1) as usize] as i32) == 2));
    assert!((((*buf.borrow())[(2) as usize] as i32) == 1));
    assert!((((*buf.borrow())[(3) as usize] as i32) == 2));
    assert!((((*buf.borrow())[(4) as usize] as i32) == 3));
    assert!((((*buf.borrow())[(5) as usize] as i32) == 4));
    return 0;
}
