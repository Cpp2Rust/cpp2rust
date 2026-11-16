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
pub struct HasValue {
    pub value: i32,
}
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut object_stamp_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut member_stamp_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut object_3: std::cell::LazyCell<HasValue> =
    std::cell::LazyCell::new(|| unsafe { HasValue { value: 7 } });
pub unsafe fn select_object_4() -> *mut HasValue {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut object_stamp_1)) =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut object_3));
}
pub unsafe fn select_member_5() -> MemberPtrUnsafe<HasValue, i32> {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut member_stamp_2)) =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    return <MemberPtrUnsafe<HasValue, i32>>::from_offset(::std::mem::offset_of!(HasValue, value));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(unsafe { select_object_4() }).member((unsafe { select_member_5() }))) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut object_stamp_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut member_stamp_2)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
    std::cell::LazyCell::force(&*&raw const object_stamp_1);
    std::cell::LazyCell::force(&*&raw const member_stamp_2);
    std::cell::LazyCell::force(&*&raw const object_3);
}
