extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasMemberNamedLikeParameter {
    pub amount: i32,
}
impl HasMemberNamedLikeParameter {
    pub unsafe fn new(mut amount: i32) -> Self {
        let mut this = Self {
            amount: ((amount) + (1)),
        };
        amount += 10;
        assert!(((amount) == (14)));
        assert!(((this.amount) == (5)));
        this
    }
    pub unsafe fn add(&self, mut amount: i32) -> i32 {
        return ((self.amount) + (amount));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasMemberNamedLikeParameter = HasMemberNamedLikeParameter::new({ 4 });
    assert!(((object.amount) == (5)));
    assert!(((unsafe { HasMemberNamedLikeParameter::add(&object, 20,) }) == (25)));
    assert!(((object.amount) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
