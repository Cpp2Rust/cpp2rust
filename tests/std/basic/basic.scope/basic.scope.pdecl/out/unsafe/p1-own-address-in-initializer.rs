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
pub struct SelfLinked {
    pub next: *mut SelfLinked,
    pub value: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut node: SelfLinked = SelfLinked {
        next: std::ptr::null_mut(),
        value: 3,
    };
    let mut outer: *mut SelfLinked = (&mut node as *mut SelfLinked);
    {
        let mut node: SelfLinked = <SelfLinked>::default();
        node = SelfLinked {
            next: (&mut node as *mut SelfLinked),
            value: 7,
        };
        assert!(((node.next) == (&mut node as *mut SelfLinked)));
        assert!(((node.next) != (outer)));
        assert!((((*node.next).value) == (7)));
    }
    assert!((node.next).is_null());
    assert!(((node.value) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
