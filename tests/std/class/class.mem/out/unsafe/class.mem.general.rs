extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut stock_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn minimum_1() -> i32 {
    return 50;
}
pub type anon_2 = u32;
pub const anon_2_reserve: anon_2 = 30;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p7_1_function_body_names_later_member_HasLaterMembers {
    pub stock: i32,
}
impl p7_1_function_body_names_later_member_HasLaterMembers {
    pub unsafe fn take(&mut self, mut amount: i32) -> i32 {
        self.stock -= amount;
        return (((self.stock)
            + (unsafe { p7_1_function_body_names_later_member_HasLaterMembers::minimum(self) }))
            + (anon_2_reserve as i32));
    }
    pub unsafe fn minimum(&self) -> i32 {
        return 2;
    }
}
impl Default for p7_1_function_body_names_later_member_HasLaterMembers {
    fn default() -> Self {
        unsafe { p7_1_function_body_names_later_member_HasLaterMembers { stock: 9 } }
    }
}
pub unsafe fn test_3() {
    let mut object: p7_1_function_body_names_later_member_HasLaterMembers =
        <p7_1_function_body_names_later_member_HasLaterMembers>::default();
    assert!(
        ((unsafe { p7_1_function_body_names_later_member_HasLaterMembers::take(&mut object, 3,) })
            == (38))
    );
    assert!(((object.stock) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut stock_0)) == (100)));
}
pub static mut step_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub static mut step_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p7_2_default_argument_names_later_member_HasLaterStaticStep {
    pub total: i32,
}
impl p7_2_default_argument_names_later_member_HasLaterStaticStep {
    pub unsafe fn advance(&mut self, mut amount: Option<i32>) -> i32 {
        let mut amount: i32 = amount
            .unwrap_or_else(|| unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut step_5)) });
        self.total += amount;
        return self.total;
    }
}
impl Default for p7_2_default_argument_names_later_member_HasLaterStaticStep {
    fn default() -> Self {
        unsafe { p7_2_default_argument_names_later_member_HasLaterStaticStep { total: 1 } }
    }
}
pub unsafe fn test_6() {
    let mut object: p7_2_default_argument_names_later_member_HasLaterStaticStep =
        <p7_2_default_argument_names_later_member_HasLaterStaticStep>::default();
    assert!(
        ((unsafe {
            p7_2_default_argument_names_later_member_HasLaterStaticStep::advance(&mut object, None)
        }) == (5))
    );
    assert!(
        ((unsafe {
            p7_2_default_argument_names_later_member_HasLaterStaticStep::advance(
                &mut object,
                Some(2),
            )
        }) == (7))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut step_4)) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_3() });
    (unsafe { test_6() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const stock_0);
    std::cell::LazyCell::force(&*&raw const step_4);
    std::cell::LazyCell::force(&*&raw const step_5);
}
