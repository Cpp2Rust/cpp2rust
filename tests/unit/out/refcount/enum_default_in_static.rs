extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Mode = u32;
pub const Mode_MODE_NONE: Mode = 0;
pub const Mode_MODE_ONE: Mode = 1;
pub const Mode_MODE_TWO: Mode = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Config {
    #[offset(0)]
    pub count: i32,
    #[offset(4)]
    pub mode: Mode,
}
thread_local!(
    pub static config_0: Value<Config> = <Value<Config>>::default();
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((({ (*config_0.with(Value::clone).borrow()).count } == 0) as i32) != 0));
    assert!(
        (((({ (*config_0.with(Value::clone).borrow()).mode } as u32)
            == ((Mode_MODE_NONE as i32) as u32)) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = config_0.with(|_| ());
}
