extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Precedes {
    pub preceding: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresValue {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InheritsValue {
    pub own: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pm: MemberPtr<DeclaresValue, i32> =
        <MemberPtr<DeclaresValue, i32>>::from_offset(::std::mem::offset_of!(DeclaresValue, value));
    let mut obj: InheritsValue = <InheritsValue>::default();
    (obj as Precedes).preceding = 1;
    (obj as DeclaresValue).value = 2;
    obj.own = 3;
    let mut p: *mut InheritsValue = (&mut obj as *mut InheritsValue);
    assert!(((*(p).member(pm)) == (2)));
    (*(p).member(pm)) = 4;
    assert!((((obj as DeclaresValue).value) == (4)));
    assert!((((obj as Precedes).preceding) == (1)));
    assert!(((obj.own) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
