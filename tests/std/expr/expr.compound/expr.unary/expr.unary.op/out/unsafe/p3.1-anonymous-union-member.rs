extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub as_int: i32,
    pub as_unsigned: u32,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasAnonymousUnion {
    pub before: i32,
    pub anon_0: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pm: isize = (::std::mem::offset_of!(HasAnonymousUnion, anon_0) as isize);
    let mut obj: HasAnonymousUnion = <HasAnonymousUnion>::default();
    obj.before = 1;
    obj.anon_0.as_int = 2;
    assert!(
        ((*(&mut obj as *mut HasAnonymousUnion)
            .wrapping_byte_offset(pm)
            .cast::<i32>())
            == (2))
    );
    (*(&mut obj as *mut HasAnonymousUnion)
        .wrapping_byte_offset(pm)
        .cast::<i32>()) = 3;
    assert!(((obj.anon_0.as_int) == (3)));
    assert!(((obj.before) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
