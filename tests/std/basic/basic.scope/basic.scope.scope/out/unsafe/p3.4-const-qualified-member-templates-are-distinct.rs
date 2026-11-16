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
pub struct HasConstTemplateOverload {
    pub plain_calls: i32,
    pub const_calls: i32,
}
impl HasConstTemplateOverload {
    pub unsafe fn get_1_int(&mut self, mut value: i32) -> i32 {
        self.plain_calls.prefix_inc();
        return ((value) + (1));
    }
    pub unsafe fn get_2_int(&self, mut value: i32) -> i32 {
        (*(&raw const self.const_calls).cast_mut()).prefix_inc();
        return ((value) + (2));
    }
}
impl Default for HasConstTemplateOverload {
    fn default() -> Self {
        unsafe {
            HasConstTemplateOverload {
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
    let mut object: HasConstTemplateOverload = <HasConstTemplateOverload>::default();
    let view: *const HasConstTemplateOverload = &object;
    assert!(((unsafe { HasConstTemplateOverload::get_1_int(&mut object, 10,) }) == (11)));
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (0)));
    assert!(((unsafe { HasConstTemplateOverload::get_2_int(&(*view), 10,) }) == (12)));
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
