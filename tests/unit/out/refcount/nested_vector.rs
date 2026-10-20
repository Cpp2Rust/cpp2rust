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
    let v1: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: Vec<i32> = Vec::from([1]);
    {
        let a0_clone = v2.clone();
        (*v1.borrow_mut()).push(a0_clone)
    };
    assert!(
        ((((v1.as_pointer() as Ptr<Vec<Vec<i32>>>).decay().to_last() as Ptr<Vec<i32>>)
            .decay()
            .offset(0_usize as isize)
            .read())
            == 1)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
