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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) = match (
            {
                match (
                    {
                        match (
                            { 0 },
                            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)),
                        ) {
                            (__tmp_4, __tmp_5) => {
                                (*__tmp_5) = __tmp_4;
                                (*__tmp_5).clone()
                            }
                        }
                    },
                    &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)),
                ) {
                    (__tmp_2, __tmp_3) => {
                        (*__tmp_3) = __tmp_2;
                        (*__tmp_3).clone()
                    }
                }
            },
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)),
        ) {
            (__tmp_0, __tmp_1) => {
                (*__tmp_1) = __tmp_0;
                (*__tmp_1).clone()
            }
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
pub unsafe fn instance_4() -> *mut LifetimeTracked {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut t_5));
}
static mut t_5: std::cell::LazyCell<LifetimeTracked> = std::cell::LazyCell::new(|| unsafe {
    {
        let __static = LifetimeTracked::new();
        libcc2rs::register_static_destructor(|| {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut t_5)).destructor()
        });
        __static
    }
});
pub static mut check_6: std::cell::LazyCell<AssertsDtorCountAtExit> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let __static = AssertsDtorCountAtExit { expected: 1 };
            libcc2rs::register_static_destructor(|| {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut check_6)).destructor()
            });
            __static
        }
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (0)));
    (unsafe { LifetimeTracked::use_(&(*(unsafe { instance_4() }))) });
    (unsafe { b_use_7() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (2)));
    assert!((((*(unsafe { instance_4() })).uses) == (2)));
    assert!(((unsafe { instance_4() }) == (unsafe { b_address_8() })));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_1)) == (0)));
    return 0;
}
pub unsafe fn b_use_7() {
    (unsafe { LifetimeTracked::use_(&(*(unsafe { instance_4() }))) });
}
pub unsafe fn b_address_8() -> *mut LifetimeTracked {
    return (unsafe { instance_4() });
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const check_6);
}
