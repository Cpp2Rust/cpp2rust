extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct HasTwoIntMembers {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl HasTwoIntMembers {
    pub fn first(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn second(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
}
impl Default for HasTwoIntMembers {
    fn default() -> Self {
        HasTwoIntMembers {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut first_pm: isize = 0_isize;
    let mut second_pm: isize = 0_isize;
    assert!((first_pm == second_pm));
    assert!(!(first_pm != second_pm));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
