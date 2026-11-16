extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut pm_0: std::cell::LazyCell<MemberPtrUnsafe<CompletedLater, i32>> =
    std::cell::LazyCell::new(|| unsafe { <MemberPtrUnsafe<CompletedLater, i32>>::default() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CompletedLater {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut pm_0)).is_null());
    (*std::cell::LazyCell::force_mut(&mut *&raw mut pm_0)) =
        <MemberPtrUnsafe<CompletedLater, i32>>::from_offset(::std::mem::offset_of!(
            CompletedLater,
            second
        ));
    let mut obj: CompletedLater = CompletedLater {
        first: 3,
        second: 4,
    };
    assert!(
        ((*(&mut obj as *mut CompletedLater)
            .member((*std::cell::LazyCell::force_mut(&mut *&raw mut pm_0))))
            == (4))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const pm_0);
}
