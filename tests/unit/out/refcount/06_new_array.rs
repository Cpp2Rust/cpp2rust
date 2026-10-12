extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn take_0(p: Ptr<Ptr<i8>>) -> i8 {
    elem!((p.read()), 0).write(9_i8);
    let mut c: i8 = (elem!((p.read()), 0).read());
    (p.read()).delete();
    return c;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut e: Ptr<i32> = Ptr::alloc_array((0..2_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    elem!(e, 0).write(6);
    elem!(e, 1).write(7);
    e.delete();
    let mut c: i8 = ({
        let _p: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::alloc_array(
            (0..4_usize).map(|_| 0_i8).collect::<Box<[i8]>>(),
        )));
        take_0(_p.as_pointer())
    });
    &(c);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
