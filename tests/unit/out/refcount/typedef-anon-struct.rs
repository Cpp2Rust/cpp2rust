extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_RunInfo {
    #[offset(0)]
    pub block_idx: i32,
    #[offset(4)]
    pub num_extra_zero_runs: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub runs: Value<Vec<Outer_RunInfo>>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut o: Outer = <Outer>::default();
    let mut info: Outer_RunInfo = <Outer_RunInfo>::default();
    info.block_idx = 1;
    info.num_extra_zero_runs = 2;
    {
        let a0_clone = info.clone();
        (*o.runs.borrow_mut()).push(a0_clone)
    };
    assert!(((*o.runs.borrow()).len() == 1_usize));
    assert!(
        ({
            (*elem!((o.runs.as_pointer() as Ptr<Outer_RunInfo>), 0_usize)
                .upgrade()
                .deref())
            .block_idx
        } == 1)
    );
    assert!(
        ({
            (*elem!((o.runs.as_pointer() as Ptr<Outer_RunInfo>), 0_usize)
                .upgrade()
                .deref())
            .num_extra_zero_runs
        } == 2)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
