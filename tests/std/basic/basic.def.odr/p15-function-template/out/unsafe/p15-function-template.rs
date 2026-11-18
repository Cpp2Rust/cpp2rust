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
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) = {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) = 0;
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3))
                };
                (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1))
        };
    }
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
pub unsafe fn count_4() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut n_6)).prefix_inc());
}
pub unsafe fn count_7() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut n_9)).prefix_inc());
}
pub unsafe fn count_10() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut n_12)).prefix_inc());
}
static mut t_5: std::cell::LazyCell<LifetimeTracked> =
    std::cell::LazyCell::new(|| unsafe { LifetimeTracked::new() });
static mut n_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut t_8: std::cell::LazyCell<LifetimeTracked> =
    std::cell::LazyCell::new(|| unsafe { LifetimeTracked::new() });
static mut n_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut t_11: std::cell::LazyCell<LifetimeTracked> =
    std::cell::LazyCell::new(|| unsafe { LifetimeTracked::new() });
static mut n_12: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut check_13: std::cell::LazyCell<AssertsDtorCountAtExit> =
    std::cell::LazyCell::new(|| unsafe { AssertsDtorCountAtExit { expected: 4 } });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (0)));
    assert!(((unsafe { count_4() }) == (1)));
    assert!(((unsafe { b_count_local_15() }) == (1)));
    assert!(((unsafe { count_4() }) == (2)));
    assert!(((unsafe { count_7() }) == (1)));
    assert!(((unsafe { b_count_int_16() }) == (2)));
    assert!(((unsafe { count_7() }) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (3)));
    assert!(((unsafe { count_10() }) == (1)));
    assert!(((unsafe { b_count_long_17() }) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (4)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_14 {}
pub unsafe fn count_18() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut n_20)).prefix_inc());
}
static mut t_19: std::cell::LazyCell<LifetimeTracked> =
    std::cell::LazyCell::new(|| unsafe { LifetimeTracked::new() });
static mut n_20: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn b_count_int_16() -> i32 {
    return (unsafe { count_7() });
}
pub unsafe fn b_count_long_17() -> i32 {
    return (unsafe { count_10() });
}
pub unsafe fn b_count_local_15() -> i32 {
    return (unsafe { count_18() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_21 {}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const check_13);
}
