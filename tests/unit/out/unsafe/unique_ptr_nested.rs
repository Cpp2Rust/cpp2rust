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
pub struct Inner {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub inner: Option<Box<Inner>>,
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self {
            inner: (*_a0).inner.take(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        self.inner = (*_a0).inner.take();
        return &mut (*(self as *mut Outer));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Option<Box<Outer>> = Some(Box::new(Outer {
        inner: Some(Box::new(Inner { x: 10, y: 20 })),
    }));
    (*(*o
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const Outer as *mut Outer))
    .inner
    .as_deref()
    .map_or(::std::ptr::null_mut(), |v| v as *const Inner as *mut Inner))
    .x += 5;
    let mut sum: i32 = (((*(*o
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const Outer as *mut Outer))
    .inner
    .as_deref()
    .map_or(::std::ptr::null_mut(), |v| v as *const Inner as *mut Inner))
    .x) + ((*(*o
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const Outer as *mut Outer))
    .inner
    .as_deref()
    .map_or(::std::ptr::null_mut(), |v| v as *const Inner as *mut Inner))
    .y));
    let mut a: Option<Box<i32>> = Some(Box::new(100));
    let mut b: Option<Box<i32>> = Some(Box::new(0));
    (*b.as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)) = (*a
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32));
    assert!(
        (((sum)
            + (*b
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)))
            == (135))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
