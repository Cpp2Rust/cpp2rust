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
    let arr: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal("foo")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    (arr.as_pointer() as Ptr<u8>)
        .offset(1_u64 as isize)
        .write(('b' as u8));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u8>).offset((1) as isize),
    ));
    assert!(((((*p.borrow()).read()) as i32) == (('b' as u8) as i32)));
    assert!((*arr.borrow())
        .iter()
        .copied()
        .take((*arr.borrow()).len() - 1)
        .eq(Ptr::from_string_literal("fbo").to_c_string_iterator()));
    return 0;
}
