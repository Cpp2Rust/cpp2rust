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
pub static mut then_runs_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut else_runs_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn test_6() {
    (unsafe { LifetimeTracked::reset() });
    let mut choice: i32 = 9;
    {
        let mut guard: LifetimeTracked = LifetimeTracked::new();
        let _dtor_guard =
            std::mem::DropGuard::new(&raw mut guard, |__p| unsafe { (*__p).destructor() });
        let mut choice: i32 = 0;
        if (choice != 0) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)).prefix_inc();
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_5)).prefix_inc();
            (unsafe { LifetimeTracked::use_(&guard) });
            assert!(((guard.uses) == (1)));
            assert!(((choice) == (0)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (0)));
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_5)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
    {
        let mut guard: LifetimeTracked = LifetimeTracked::new();
        let _dtor_guard =
            std::mem::DropGuard::new(&raw mut guard, |__p| unsafe { (*__p).destructor() });
        let mut choice: i32 = 4;
        if (choice != 0) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)).prefix_inc();
            (unsafe { LifetimeTracked::use_(&guard) });
            assert!(((guard.uses) == (1)));
            assert!(((choice) == (4)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (2)));
    assert!(((choice) == (9)));
}
pub unsafe fn pick_7() -> i32 {
    {
        return 2;
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn only_then_8() -> i32 {
    let mut r: i32 = 3;
    {}
    return r;
}
pub unsafe fn test_9() {
    assert!(((unsafe { pick_7() }) == (2)));
    assert!(((unsafe { only_then_8() }) == (3)));
}
pub unsafe fn pick_10() -> i32 {
    {
        return 2;
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn only_then_11() -> i32 {
    let mut r: i32 = 3;
    {
        r = 4;
    }
    return r;
}
pub unsafe fn test_12() {
    assert!(((unsafe { pick_10() }) == (2)));
    assert!(((unsafe { only_then_11() }) == (4)));
}
pub unsafe fn classify_13(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    if ((x) > (0)) {
        r = 1;
    } else {
        std::hint::cold_path();
    }
    if ((x) < (-10_i32)) {
        std::hint::cold_path();
        r += 2;
    } else {
        r += 3;
    }
    if ((x) == (5)) {
        std::hint::cold_path();
        r += 4;
    } else {
        r += 5;
    }
    if ((x) == (7)) {
        r = 7;
    } else {
        std::hint::cold_path();
        if ((x) == (8)) {
            r = 8;
        }
    }
    return r;
}
pub unsafe fn test_14() {
    assert!(((unsafe { classify_13(1,) }) == (9)));
    assert!(((unsafe { classify_13(5,) }) == (8)));
    assert!(((unsafe { classify_13(-20_i32,) }) == (7)));
    assert!(((unsafe { classify_13(7,) }) == (7)));
    assert!(((unsafe { classify_13(8,) }) == (8)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_14() });
    (unsafe { test_6() });
    (unsafe { test_9() });
    (unsafe { test_12() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const then_runs_4);
    std::cell::LazyCell::force(&*&raw const else_runs_5);
}
