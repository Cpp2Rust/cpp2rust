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
    pub namespace_count: i32,
    pub namespace_ratio: f64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub member_count: i32,
    pub member_ratio: f64,
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasAnonymousUnion {
    pub tag: i32,
    pub anon_1: anon_1,
}
impl HasAnonymousUnion {
    pub unsafe fn read_count(&self) -> i32 {
        return self.anon_1.member_count;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_0)).namespace_count = 4;
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut anon_0)).namespace_count) == (4)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_0)).namespace_ratio = 0.25_f64;
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut anon_0)).namespace_ratio) == (0.25_f64))
    );
    let mut object: HasAnonymousUnion = <HasAnonymousUnion>::default();
    object.tag = 1;
    object.anon_1.member_count = 6;
    assert!(((object.anon_1.member_count) == (6)));
    assert!(((unsafe { HasAnonymousUnion::read_count(&object,) }) == (6)));
    object.anon_1.member_ratio = 0.5_f64;
    assert!(((object.anon_1.member_ratio) == (0.5_f64)));
    assert!(((object.tag) == (1)));
    let mut anon_2: anon_2 = <anon_2>::default();
    anon_2.block_count = 9;
    assert!(((anon_2.block_count) == (9)));
    anon_2.block_ratio = 0.75_f64;
    assert!(((anon_2.block_ratio) == (0.75_f64)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_2 {
    pub block_count: i32,
    pub block_ratio: f64,
}
impl Default for anon_2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
