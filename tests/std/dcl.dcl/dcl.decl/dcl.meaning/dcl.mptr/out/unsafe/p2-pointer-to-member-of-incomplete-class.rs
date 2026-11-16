extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut pm_0: std::cell::LazyCell<isize> = std::cell::LazyCell::new(|| unsafe { -1_isize });
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
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut pm_0)) == -1_isize));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut pm_0)) =
        (::std::mem::offset_of!(CompletedLater, second) as isize);
    let mut obj: CompletedLater = CompletedLater {
        first: 3,
        second: 4,
    };
    assert!(
        ((*(&mut obj as *mut CompletedLater)
            .wrapping_byte_offset((*std::cell::LazyCell::force_mut(&mut *&raw mut pm_0)))
            .cast::<i32>())
            == (4))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const pm_0);
}
