extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasConstOverload {
    pub plain_calls: i32,
    pub const_calls: i32,
}
impl HasConstOverload {
    pub unsafe fn get_1(&mut self) -> i32 {
        self.plain_calls.prefix_inc();
        return 1;
    }
    pub unsafe fn get_2(&self) -> i32 {
        (*(&raw const self.const_calls).cast_mut()).prefix_inc();
        return 2;
    }
}
impl Default for HasConstOverload {
    fn default() -> Self {
        unsafe {
            HasConstOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasConstOverload = <HasConstOverload>::default();
    let view: *const HasConstOverload = &object;
    assert!(((unsafe { HasConstOverload::get_1(&mut object,) }) == (1)));
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (0)));
    assert!(((unsafe { HasConstOverload::get_2(&(*view),) }) == (2)));
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
