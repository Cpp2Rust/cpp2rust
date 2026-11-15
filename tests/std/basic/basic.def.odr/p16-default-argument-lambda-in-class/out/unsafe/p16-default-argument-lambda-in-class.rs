extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctorCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copyCount_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut dtorCount_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut useCount_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeTracked {
    pub uses: i32,
}
impl LifetimeTracked {
    pub unsafe fn new() -> Self {
        let mut this = Self { uses: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const LifetimeTracked) -> Self {
        let mut this = Self { uses: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)).prefix_inc();
    }
    pub unsafe fn use_(&self) {
        (*(&raw const self.uses).cast_mut()).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const LifetimeTracked) -> *mut LifetimeTracked {
        self.uses = (*_a0).uses;
        return &mut (*(self as *mut LifetimeTracked));
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        unsafe { LifetimeTracked::copy_from(self as *const LifetimeTracked) }
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        unsafe { LifetimeTracked::new() }
    }
}
pub static mut copyCount_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasConstexprCtor {
    pub m: i32,
}
impl HasConstexprCtor {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { m: v };
        this
    }
    pub unsafe fn copy_from(o: *const HasConstexprCtor) -> Self {
        let mut this = Self { m: (*o).m };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_4)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const HasConstexprCtor) -> *mut HasConstexprCtor {
        self.m = (*_a0).m;
        return &mut (*(self as *mut HasConstexprCtor));
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        unsafe { HasConstexprCtor::copy_from(self as *const HasConstexprCtor) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct AssertsDtorCountAtExit {
    pub expected: i32,
}
impl AssertsDtorCountAtExit {
    pub unsafe fn destructor(&mut self) {
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (self.expected)));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {}
impl X {
    pub unsafe fn h(&mut self, mut cond: bool, mut p: Option<Option<unsafe fn()>>) {
        let mut p: Option<unsafe fn()> = p.unwrap_or_else(|| unsafe {
            Some(|| unsafe {
                (unsafe {
                    LifetimeTracked::use_(
                        &(*std::cell::LazyCell::force_mut(&mut *&raw mut local_5)),
                    )
                });
            })
        });
        if cond {
            (unsafe { X::h(self, false, None) });
        } else {
            (unsafe { (p).unwrap()() });
        }
    }
}
static mut local_5: std::cell::LazyCell<LifetimeTracked> =
    std::cell::LazyCell::new(|| unsafe { LifetimeTracked::new() });
pub static mut check_6: std::cell::LazyCell<AssertsDtorCountAtExit> =
    std::cell::LazyCell::new(|| unsafe { AssertsDtorCountAtExit { expected: 1 } });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (0)));
    let mut x: X = <X>::default();
    (unsafe { X::h(&mut x, true, None) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (1)));
    (unsafe { b_call_7() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (2)));
    return 0;
}
pub unsafe fn b_call_7() {
    let mut x: X = <X>::default();
    (unsafe { X::h(&mut x, true, None) });
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const copyCount_4);
    std::cell::LazyCell::force(&*&raw const check_6);
}
