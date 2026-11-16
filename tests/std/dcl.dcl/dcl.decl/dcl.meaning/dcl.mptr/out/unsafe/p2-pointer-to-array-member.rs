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
pub struct HasIntArray {
    pub before: i32,
    pub elements: [i32; 3],
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pm: MemberPtrUnsafe<HasIntArray, [i32; 3]> =
        <MemberPtrUnsafe<HasIntArray, [i32; 3]>>::from_offset(::std::mem::offset_of!(
            HasIntArray,
            elements
        ));
    let mut obj: HasIntArray = HasIntArray {
        before: 1,
        elements: [2, 3, 4],
    };
    assert!((((*(&mut obj as *mut HasIntArray).member(pm))[(0) as usize]) == (2)));
    assert!((((*(&mut obj as *mut HasIntArray).member(pm))[(2) as usize]) == (4)));
    (*(&mut obj as *mut HasIntArray).member(pm))[(1) as usize] = 7;
    assert!(((obj.elements[(1) as usize]) == (7)));
    assert!(((obj.before) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
