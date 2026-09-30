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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub runs: Value<Vec<Outer_RunInfo>>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        Self {
            runs: Rc::new(RefCell::new((*self.runs.borrow()).clone())),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    let info: Value<Outer_RunInfo> = Rc::new(RefCell::new(<Outer_RunInfo>::default()));
    (*info.borrow_mut()).block_idx = 1;
    (*info.borrow_mut()).num_extra_zero_runs = 2;
    {
        let a0_clone = (*info.borrow()).clone();
        (*{ (*o.borrow()).runs.clone() }.borrow_mut()).push(a0_clone)
    };
    assert!(((*{ (*o.borrow()).runs.clone() }.borrow()).len() == 1_usize));
    assert!(
        ({
            (*({ (*o.borrow()).runs.clone() }.as_pointer() as Ptr<Outer_RunInfo>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .block_idx
        } == 1)
    );
    assert!(
        ({
            (*({ (*o.borrow()).runs.clone() }.as_pointer() as Ptr<Outer_RunInfo>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .num_extra_zero_runs
        } == 2)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
