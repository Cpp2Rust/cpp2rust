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
pub struct p18_rebind_to_container_node_Alloc_std_pair_const_int__int__ {}
pub unsafe fn test_0() {
    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) = 2;
    assert!(((*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) == (2)));
    assert!(((m.len()) == (1_u64)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
