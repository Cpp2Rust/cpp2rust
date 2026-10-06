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
    let mut g: Option<Value<Box<[i32]>>> = Some(Rc::new(RefCell::new(
        (0..2_usize).map(|_| <i32>::default()).collect::<Box<[_]>>(),
    )));
    g.as_ref().unwrap().borrow_mut()[(0_usize) as usize] = 11;
    g.as_ref().unwrap().borrow_mut()[(1_usize) as usize] = 12;
    let mut g_ptr: Ptr<i32> = g.as_pointer();
    elem!(g_ptr, 0).write(13);
    elem!(g_ptr, 1).write(14);
    assert!(
        ((g.as_ref().unwrap().borrow()[(0_usize) as usize]
            + g.as_ref().unwrap().borrow()[(1_usize) as usize])
            == 27)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
