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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 7;
        (*v.borrow_mut()).push(__a1)
    };
    let mut it: Ptr<i32> = {
        let count = ((v.as_pointer() as Ptr<i32>).to_end().get_offset()
            - (v.as_pointer() as Ptr<i32>).get_offset()) as usize;
        (v.as_pointer() as Ptr<i32>).offset(
            (v.as_pointer() as Ptr<i32>)
                .clone()
                .into_iter()
                .take(count)
                .position(|value_0| value_0.read() == 7)
                .unwrap_or(count) as isize,
        )
    };
    assert!(it != (v.as_pointer() as Ptr<i32>).to_end());
    assert!(((it.read()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
