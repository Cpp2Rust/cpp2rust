extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut a0: Ptr<i32>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(
        (0..(4_usize) as usize)
            .map(|_| <i32>::default())
            .collect::<Vec<_>>(),
    ));
    let mut it: Ptr<i32> = (vec_.as_pointer() as Ptr<i32>);
    ({ foo_0((it).clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
