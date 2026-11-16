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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HoldsSelector {
    pub selector: isize,
}
impl Default for HoldsSelector {
    fn default() -> Self {
        unsafe { HoldsSelector { selector: -1_isize } }
    }
}
pub static mut static_pm_0: std::cell::LazyCell<isize> =
    std::cell::LazyCell::new(|| unsafe { -1_isize });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut static_pm_0)) == -1_isize));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_pm_0))
            != (::std::mem::offset_of!(HasTwoInts, first) as isize))
    );
    let mut value_initialized: isize = -1_isize;
    assert!(((value_initialized) == -1_isize));
    assert!(((value_initialized) != (::std::mem::offset_of!(HasTwoInts, first) as isize)));
    let mut holder: HoldsSelector = <HoldsSelector>::default();
    assert!(((holder.selector) == -1_isize));
    assert!(((holder.selector) != (::std::mem::offset_of!(HasTwoInts, first) as isize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const static_pm_0);
}
