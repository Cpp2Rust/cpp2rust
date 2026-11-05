extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[inline]
pub unsafe fn inlined_free_0(mut a: i32) -> i32 {
    return ((a) + (1));
}
#[inline]
pub unsafe fn inlined_constexpr_free_1(mut a: i32) -> i32 {
    return ((a) + (2));
}
#[inline]
pub unsafe fn inlined_declared_free_2(mut a: i32) -> i32 {
    return ((a) + (3));
}
pub unsafe fn not_inlined_free_3(mut a: i32) -> i32 {
    return ((a) + (4));
}
pub unsafe fn not_inlined_static_free_4(mut a: i32) -> i32 {
    return ((a) + (5));
}
pub unsafe trait Base {
    unsafe fn inlined_override(&self, a: i32) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Derived {
    v_: i32,
}
impl Derived {
    #[inline]
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v_: v };
        this
    }
    #[inline]
    pub unsafe fn inlined_in_class(&self) -> i32 {
        return ((self.v_) * (2));
    }
    #[inline]
    pub unsafe fn inlined_static_in_class(mut a: i32) -> i32 {
        return ((a) * (3));
    }
}
impl Derived {
    #[inline]
    pub unsafe fn inlined_declared_in_class(&self) -> i32 {
        return ((self.v_) * (4));
    }
}
impl Derived {
    #[inline]
    pub unsafe fn inlined_defined_out_of_line(&self) -> i32 {
        return ((self.v_) * (5));
    }
}
impl Derived {
    pub unsafe fn not_inlined_out_of_line(&self) -> i32 {
        return ((self.v_) * (6));
    }
}
impl Derived {
    pub unsafe fn not_inlined_static_out_of_line(mut a: i32) -> i32 {
        return ((a) * (7));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: Derived = Derived::new({ 3 });
    assert!(((unsafe { inlined_free_0(1,) }) == (2)));
    assert!(((unsafe { inlined_constexpr_free_1(1,) }) == (3)));
    assert!(((unsafe { inlined_declared_free_2(1,) }) == (4)));
    assert!(((unsafe { not_inlined_free_3(1,) }) == (5)));
    assert!(((unsafe { not_inlined_static_free_4(1,) }) == (6)));
    assert!(((unsafe { d.inlined_override(1,) }) == (4)));
    assert!(((unsafe { Derived::inlined_in_class(&d,) }) == (6)));
    assert!(((unsafe { Derived::inlined_static_in_class(2,) }) == (6)));
    assert!(((unsafe { Derived::inlined_declared_in_class(&d,) }) == (12)));
    assert!(((unsafe { Derived::inlined_defined_out_of_line(&d,) }) == (15)));
    assert!(((unsafe { Derived::not_inlined_out_of_line(&d,) }) == (18)));
    assert!(((unsafe { Derived::not_inlined_static_out_of_line(2,) }) == (14)));
    return 0;
}
unsafe impl Base for Derived {
    #[inline]
    unsafe fn inlined_override(&self, a: i32) -> i32 {
        return ((self.v_) + (a));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
