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
    let buf: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8, 5_i8, 6_i8])));
    {
        ((buf.as_pointer() as Ptr<i8>).offset((2) as isize) as Ptr<i8>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                4_usize as usize,
            );
        ((buf.as_pointer() as Ptr<i8>).offset((2) as isize) as Ptr<i8>).to_any()
    };
    assert!((((*buf.borrow())[(0) as usize] as i32) == 1));
    assert!((((*buf.borrow())[(1) as usize] as i32) == 2));
    assert!((((*buf.borrow())[(2) as usize] as i32) == 1));
    assert!((((*buf.borrow())[(3) as usize] as i32) == 2));
    assert!((((*buf.borrow())[(4) as usize] as i32) == 3));
    assert!((((*buf.borrow())[(5) as usize] as i32) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
