extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut scaled_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut rate_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn scaled_2(mut amount: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut scaled_calls_0)).prefix_inc();
    return ((amount) * (10));
}
pub unsafe fn total_3(mut base: Option<i32>) -> i32 {
    let mut base: i32 = base.unwrap_or_else(|| unsafe {
        (unsafe { scaled_2((*std::cell::LazyCell::force_mut(&mut *&raw mut rate_1))) })
    });
    return base;
}
pub unsafe fn test_4() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut rate_1)) = 2;
    {
        let mut rate: i32 = 3;
        assert!(((unsafe { total_3(None,) }) == (20)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut scaled_calls_0)) == (1)));
        assert!(((rate) == (3)));
    }
    assert!(((unsafe { total_3(Some(5),) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut scaled_calls_0)) == (1)));
}
pub static mut next_ticket_calls_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn next_ticket_6() -> i32 {
    return (((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_5)).prefix_inc())
        * (100));
}
pub unsafe fn issue_7(mut ticket: Option<i32>) -> i32 {
    let mut ticket: i32 = ticket.unwrap_or_else(|| unsafe { (unsafe { next_ticket_6() }) });
    return ticket;
}
pub unsafe fn test_8() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_5)) == (0)));
    assert!(((unsafe { issue_7(None,) }) == (100)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_5)) == (1)));
    assert!(((unsafe { issue_7(Some(7),) }) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_5)) == (1)));
    assert!(((unsafe { issue_7(None,) }) == (200)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_5)) == (2)));
}
pub static mut step_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub static mut step_10: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_static_member_hides_namespace_variable_HasStaticStep {
    pub total: i32,
}
impl p9_static_member_hides_namespace_variable_HasStaticStep {
    pub unsafe fn advance(&mut self, mut amount: Option<i32>) -> i32 {
        let mut amount: i32 = amount.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut step_10))
        });
        self.total += amount;
        return self.total;
    }
}
impl Default for p9_static_member_hides_namespace_variable_HasStaticStep {
    fn default() -> Self {
        unsafe { p9_static_member_hides_namespace_variable_HasStaticStep { total: 1 } }
    }
}
pub unsafe fn test_11() {
    let mut object: p9_static_member_hides_namespace_variable_HasStaticStep =
        <p9_static_member_hides_namespace_variable_HasStaticStep>::default();
    assert!(
        ((unsafe {
            p9_static_member_hides_namespace_variable_HasStaticStep::advance(&mut object, None)
        }) == (5))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut step_10)) = 6;
    assert!(
        ((unsafe {
            p9_static_member_hides_namespace_variable_HasStaticStep::advance(&mut object, None)
        }) == (11))
    );
    assert!(
        ((unsafe {
            p9_static_member_hides_namespace_variable_HasStaticStep::advance(&mut object, Some(2))
        }) == (13))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut step_9)) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_4() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const scaled_calls_0);
    std::cell::LazyCell::force(&*&raw const rate_1);
    std::cell::LazyCell::force(&*&raw const next_ticket_calls_5);
    std::cell::LazyCell::force(&*&raw const step_9);
    std::cell::LazyCell::force(&*&raw const step_10);
}
