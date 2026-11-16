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
pub struct HasTwoInts {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HoldsSelector {
    pub selector: MemberPtrUnsafe<HasTwoInts, i32>,
}
pub static mut static_pm_0: std::cell::LazyCell<MemberPtrUnsafe<HasTwoInts, i32>> =
    std::cell::LazyCell::new(|| unsafe { <MemberPtrUnsafe<HasTwoInts, i32>>::default() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut static_pm_0)).is_null());
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_pm_0))
            != (<MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(
                HasTwoInts, first
            ))))
    );
    let mut value_initialized: MemberPtrUnsafe<HasTwoInts, i32> =
        <MemberPtrUnsafe<HasTwoInts, i32>>::default();
    assert!((value_initialized).is_null());
    assert!(
        ((value_initialized)
            != (<MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(
                HasTwoInts, first
            ))))
    );
    let mut holder: HoldsSelector = <HoldsSelector>::default();
    assert!((holder.selector).is_null());
    assert!(
        ((holder.selector)
            != (<MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(
                HasTwoInts, first
            ))))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const static_pm_0);
}
