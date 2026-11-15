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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: LifetimeTracked = LifetimeTracked::new();
    let _dtor_t = std::mem::DropGuard::new(&raw mut t, |__p| unsafe { (*__p).destructor() });
    let r: *mut LifetimeTracked = &mut t;
    let mut f: FnPtr<fn()> = lambda_unsafe!(
        {
            let r: LifetimeTracked = LifetimeTracked::copy_from({ r });
        },
        || {
            (unsafe { LifetimeTracked::use_(&r) });
        }
    );
    let _dtor_f = std::mem::DropGuard::new(&raw mut f, |__p| unsafe { (*__p).destroy() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)) == (1)));
    (unsafe { f.call() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (1)));
    assert!(((t.uses) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
}
