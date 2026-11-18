extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut balance_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 50 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p1_member_hides_namespace_variable_HasBalance {
    pub balance: i32,
}
impl p1_member_hides_namespace_variable_HasBalance {
    pub unsafe fn deposit(&mut self, mut amount: i32) -> i32 {
        self.balance += amount;
        return self.balance;
    }
    pub unsafe fn deposit_globally(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0)) += amount;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0));
    }
}
impl Default for p1_member_hides_namespace_variable_HasBalance {
    fn default() -> Self {
        unsafe { p1_member_hides_namespace_variable_HasBalance { balance: 7 } }
    }
}
pub unsafe fn test_1() {
    let mut object: p1_member_hides_namespace_variable_HasBalance =
        <p1_member_hides_namespace_variable_HasBalance>::default();
    assert!(
        ((unsafe { p1_member_hides_namespace_variable_HasBalance::deposit(&mut object, 3,) })
            == (10))
    );
    assert!(((object.balance) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0)) == (50)));
    assert!(
        ((unsafe {
            p1_member_hides_namespace_variable_HasBalance::deposit_globally(&mut object, 4)
        }) == (54))
    );
    assert!(((object.balance) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0)) == (54)));
}
pub static mut initial_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    pub held: i32,
}
impl Default for p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    fn default() -> Self {
        unsafe { p1_out_of_class_definition_is_in_class_scope_HasStaticTotal { held: 1 } }
    }
}
pub static mut initial_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 6 });
pub static mut total_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut initial_3)) + (1))
});
impl p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        self.held += amount;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_4)) +=
            (*std::cell::LazyCell::force_mut(&mut *&raw mut initial_3));
        return self.held;
    }
}
pub unsafe fn test_5() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_4)) == (7)));
    let mut object: p1_out_of_class_definition_is_in_class_scope_HasStaticTotal =
        <p1_out_of_class_definition_is_in_class_scope_HasStaticTotal>::default();
    assert!(
        ((unsafe {
            p1_out_of_class_definition_is_in_class_scope_HasStaticTotal::add(&mut object, 4)
        }) == (5))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_4)) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut initial_3)) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut initial_2)) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const balance_0);
    std::cell::LazyCell::force(&*&raw const initial_2);
    std::cell::LazyCell::force(&*&raw const initial_3);
    std::cell::LazyCell::force(&*&raw const total_4);
}
