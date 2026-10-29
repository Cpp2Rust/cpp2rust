extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static g_0: Value<Vec<u8>> = Rc::new(RefCell::new(
        (0..(3_usize) as usize).map(|_| 0_u8).collect::<Vec<_>>(),
    ));
);
pub fn get_1() -> Ptr<Vec<u8>> {
    return g_0.with(|v| v.as_pointer());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    index!(({ get_1() }), 1_usize).write(4_u8);
    let mut v: Option<Value<Vec<u8>>> =
        Ptr::alloc((0..(3_usize) as usize).map(|_| 0_u8).collect::<Vec<_>>()).to_owned_opt();
    elem!((v.as_pointer()).decay(), 2_usize).write(5_u8);
    assert!((((index!(({ get_1() }), 1_usize).read()) as i32) == 4));
    assert!((((elem!((v.as_pointer()).decay(), 2_usize).read()) as i32) == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_0.with(|_| ());
}
