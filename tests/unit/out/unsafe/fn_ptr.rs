extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_foo_0(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn foo_1(
    mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    mut pi: *mut i32,
) -> i32 {
    return (unsafe { (fn_).unwrap()((pi as *mut ::libc::c_void)) });
}
pub unsafe fn ret_size_2(mut v: i32) -> usize {
    return (((v) + (1)) as usize);
}
pub unsafe fn call_fn_3(mut f: Option<unsafe fn(i32) -> usize>, mut v: i32) -> usize {
    return (unsafe { (f).unwrap()(v) }).wrapping_mul(2_usize);
}
pub unsafe fn identity_hash_4(mut v: bool) -> usize {
    return (v as usize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    pub h: Option<unsafe fn(bool) -> u64>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub unsafe fn new(h: *const Option<unsafe fn(bool) -> u64>) -> Self {
        let mut this = Self { h: (*h) };
        this
    }
}
impl Default for HashHolder_unsigned_long__ptr__bool__ {
    fn default() -> Self {
        HashHolder_unsigned_long__ptr__bool__ { h: None }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32> = None;
    assert!((fn_).is_none());
    assert!(((fn_) != (Some(my_foo_0))));
    fn_ = Some(my_foo_0);
    assert!(!((fn_).is_none()));
    assert!(((fn_) == (Some(my_foo_0))));
    let mut a: i32 = 10;
    assert!(((unsafe { foo_1(fn_, (&mut a as *mut i32),) }) == (a)));
    assert!(((unsafe { call_fn_3(Some(ret_size_2), 3,) }) == (8_usize)));
    let mut hh: HashHolder_unsigned_long__ptr__bool__ = {
        let mut __tmp_0: Option<unsafe fn(bool) -> u64> = std::mem::transmute::<
            Option<unsafe fn(bool) -> usize>,
            Option<unsafe fn(bool) -> u64>,
        >(Some(identity_hash_4));
        HashHolder_unsigned_long__ptr__bool__::new({ &mut __tmp_0 })
    };
    assert!(((unsafe { (hh.h).unwrap()(true,) }) == (1_u64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
