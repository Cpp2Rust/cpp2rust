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
pub static mut dtorCount_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct p1_1_condition_variable_visible_in_else_ConvertsToBool {
    pub value: i32,
}
impl p1_1_condition_variable_visible_in_else_ConvertsToBool {
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.value) != (0));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)).prefix_inc();
    }
}
pub unsafe fn test_7() {
    {
        let mut condition: p1_1_condition_variable_visible_in_else_ConvertsToBool =
            p1_1_condition_variable_visible_in_else_ConvertsToBool { value: 0 };
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if (unsafe { p1_1_condition_variable_visible_in_else_ConvertsToBool::to_bool(&condition) })
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)).prefix_inc();
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_5)).prefix_inc();
            assert!(((condition.value) == (0)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)) == (0)));
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_5)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)) == (1)));
    {
        let mut condition: p1_1_condition_variable_visible_in_else_ConvertsToBool =
            p1_1_condition_variable_visible_in_else_ConvertsToBool { value: 6 };
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if (unsafe { p1_1_condition_variable_visible_in_else_ConvertsToBool::to_bool(&condition) })
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)).prefix_inc();
            assert!(((condition.value) == (6)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)) == (1)));
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_5)).prefix_inc();
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut then_runs_4)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_5)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)) == (2)));
}
pub unsafe fn test_8() {
    let mut index: i32 = 42;
    let mut iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (3)) {
            iterations.prefix_inc();
            sum += index;
            index.prefix_inc();
        }
    }
    assert!(((iterations) == (3)));
    assert!(((sum) == (3)));
    assert!(((index) == (42)));
}
pub unsafe fn test_9() {
    (unsafe { LifetimeTracked::reset() });
    let mut scoped: LifetimeTracked = LifetimeTracked::new();
    let _dtor_scoped =
        std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (3)) {
            let mut scoped: LifetimeTracked = LifetimeTracked::new();
            let _dtor_scoped =
                std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
            index.prefix_inc();
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (3)));
    if ((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (4)) {
        let mut scoped: LifetimeTracked = LifetimeTracked::new();
        let _dtor_scoped =
            std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (4)));
    if ((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (0)) {
    } else {
        let mut scoped: LifetimeTracked = LifetimeTracked::new();
        let _dtor_scoped =
            std::mem::DropGuard::new(&raw mut scoped, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (5)));
    (unsafe { LifetimeTracked::use_(&scoped) });
    assert!(((scoped.uses) == (1)));
}
pub unsafe fn test_10() {
    let mut depth: i32 = 1;
    let mut outermost: *mut i32 = (&mut depth as *mut i32);
    {
        let mut depth: i32 = 2;
        (*outermost) += 10;
        assert!(((depth) == (2)));
        {
            let mut depth: i32 = 3;
            (*outermost) += 100;
            assert!(((depth) == (3)));
        }
        assert!(((depth) == (2)));
    }
    assert!(((depth) == (111)));
}
pub unsafe fn test_11() {
    (unsafe { LifetimeTracked::reset() });
    let mut tracked: LifetimeTracked = LifetimeTracked::new();
    let _dtor_tracked =
        std::mem::DropGuard::new(&raw mut tracked, |__p| unsafe { (*__p).destructor() });
    {
        let mut tracked: LifetimeTracked = LifetimeTracked::new();
        let _dtor_tracked =
            std::mem::DropGuard::new(&raw mut tracked, |__p| unsafe { (*__p).destructor() });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (2)));
        (unsafe { LifetimeTracked::use_(&tracked) });
        assert!(((tracked.uses) == (1)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (0)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
    assert!(((tracked.uses) == (0)));
    (unsafe { LifetimeTracked::use_(&tracked) });
    assert!(((tracked.uses) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_3)) == (2)));
}
pub unsafe fn test_12() {
    let mut outer_iterations: i32 = 0;
    let mut inner_iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while ((index) < (2)) {
            {
                let mut index: i32 = 10;
                'loop_: while ((index) < (13)) {
                    inner_iterations.prefix_inc();
                    sum += index;
                    index.prefix_inc();
                }
            }
            {
                index.prefix_inc();
                outer_iterations.prefix_inc()
            };
        }
    }
    assert!(((outer_iterations) == (2)));
    assert!(((inner_iterations) == (6)));
    assert!(((sum) == (66)));
}
pub static mut inner_then_runs_13: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut inner_else_runs_14: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut outer_else_runs_15: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn test_16() {
    {
        let mut choice: i32 = 4;
        if (choice != 0) {
            {
                let mut choice: i32 = 0;
                if (choice != 0) {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_then_runs_13))
                        .prefix_inc();
                } else {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_else_runs_14))
                        .prefix_inc();
                    assert!(((choice) == (0)));
                }
            }
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut outer_else_runs_15)).prefix_inc();
            assert!(((choice) == (0)));
        }
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_then_runs_13)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_else_runs_14)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut outer_else_runs_15)) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_9() });
    (unsafe { test_10() });
    (unsafe { test_11() });
    (unsafe { test_12() });
    (unsafe { test_16() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const then_runs_4);
    std::cell::LazyCell::force(&*&raw const else_runs_5);
    std::cell::LazyCell::force(&*&raw const dtorCount_6);
    std::cell::LazyCell::force(&*&raw const inner_then_runs_13);
    std::cell::LazyCell::force(&*&raw const inner_else_runs_14);
    std::cell::LazyCell::force(&*&raw const outer_else_runs_15);
}
