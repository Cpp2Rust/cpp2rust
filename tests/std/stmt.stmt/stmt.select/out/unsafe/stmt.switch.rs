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
pub static mut matching_case_runs_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut other_case_runs_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut dtorCount_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct p2_condition_variable_lives_for_whole_statement_ConvertsToInt {
    pub value: i32,
}
impl p2_condition_variable_lives_for_whole_statement_ConvertsToInt {
    pub unsafe fn to_i32(&self) -> i32 {
        return self.value;
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)).prefix_inc();
    }
}
pub unsafe fn test_7() {
    {
        let mut selector: p2_condition_variable_lives_for_whole_statement_ConvertsToInt =
            p2_condition_variable_lives_for_whole_statement_ConvertsToInt { value: 2 };
        let _dtor_selector =
            std::mem::DropGuard::new(&raw mut selector, |__p| unsafe { (*__p).destructor() });
        'switch: {
            match {
                (unsafe {
                    p2_condition_variable_lives_for_whole_statement_ConvertsToInt::to_i32(&selector)
                })
            } {
                __v if __v == 2 => {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut matching_case_runs_4))
                        .prefix_inc();
                    assert!(((selector.value) == (2)));
                    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)) == (0)));
                    break 'switch;
                }
                _ => {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut other_case_runs_5))
                        .prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut matching_case_runs_4)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut other_case_runs_5)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_6)) == (1)));
}
pub static mut matching_case_runs_8: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut other_case_runs_9: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn test_10() {
    (unsafe { LifetimeTracked::reset() });
    let mut selector: i32 = 9;
    {
        let mut guard: LifetimeTracked = LifetimeTracked::new();
        let _dtor_guard =
            std::mem::DropGuard::new(&raw mut guard, |__p| unsafe { (*__p).destructor() });
        let mut selector: i32 = 2;
        'switch: {
            match { selector } {
                __v if __v == 2 => {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut matching_case_runs_8))
                        .prefix_inc();
                    (unsafe { LifetimeTracked::use_(&guard) });
                    assert!(((guard.uses) == (1)));
                    assert!(((selector) == (2)));
                    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (0)));
                    break 'switch;
                }
                _ => {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut other_case_runs_9))
                        .prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut matching_case_runs_8)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut other_case_runs_9)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
    assert!(((selector) == (9)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_7() });
    (unsafe { test_10() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const copyCount_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
    std::cell::LazyCell::force(&*&raw const useCount_3);
    std::cell::LazyCell::force(&*&raw const matching_case_runs_4);
    std::cell::LazyCell::force(&*&raw const other_case_runs_5);
    std::cell::LazyCell::force(&*&raw const dtorCount_6);
    std::cell::LazyCell::force(&*&raw const matching_case_runs_8);
    std::cell::LazyCell::force(&*&raw const other_case_runs_9);
}
