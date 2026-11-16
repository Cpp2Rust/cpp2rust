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
pub struct RecordsConstructorName {
    pub name: *const libc::c_char,
}
impl RecordsConstructorName {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            name: c"RecordsConstructorName".as_ptr(),
        };
        assert!(((this.name) == (c"RecordsConstructorName".as_ptr())));
        this
    }
}
impl Default for RecordsConstructorName {
    fn default() -> Self {
        unsafe { RecordsConstructorName::new() }
    }
}
pub unsafe fn plain_function_0() -> *const libc::c_char {
    return c"plain_function".as_ptr();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((libc::strcmp((unsafe { plain_function_0() }), c"plain_function".as_ptr())) == (0)));
    assert!(((unsafe { plain_function_0() }) == (unsafe { plain_function_0() })));
    let mut object: RecordsConstructorName = RecordsConstructorName::new();
    assert!(((libc::strcmp(object.name, c"RecordsConstructorName".as_ptr())) == (0)));
    let mut enclosing: *const libc::c_char = c"main".as_ptr();
    let mut lambda: FnPtr<fn() -> *const libc::c_char> =
        FnPtr::<fn() -> *const libc::c_char>::new(|| -> *const libc::c_char {
            unsafe {
                return c"operator()".as_ptr();
            }
        });
    assert!(((unsafe { lambda.call() }) != (enclosing)));
    assert!(((libc::strcmp(enclosing, c"main".as_ptr())) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
