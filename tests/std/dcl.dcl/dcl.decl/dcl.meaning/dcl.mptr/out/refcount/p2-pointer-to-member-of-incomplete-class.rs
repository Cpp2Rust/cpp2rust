extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static pm_0: Value<isize> = Rc::new(RefCell::new(-1_isize));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CompletedLater {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((pm_0.with(|rc| *rc.borrow())) == -1_isize));
    pm_0.with(|rc| *rc.borrow_mut() = (<CompletedLater as Record>::OFFSETS.second as isize));
    let obj: Value<CompletedLater> = Rc::new(RefCell::new(CompletedLater {
        first: 3,
        second: 4,
    }));
    assert!(
        (((obj.as_pointer())
            .member::<i32>(pm_0.with(|rc| *rc.borrow()))
            .read())
            == 4)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = pm_0.with(|_| ());
}
