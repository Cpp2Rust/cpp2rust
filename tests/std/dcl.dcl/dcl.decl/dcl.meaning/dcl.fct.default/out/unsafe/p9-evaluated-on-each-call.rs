extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut next_ticket_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn next_ticket_1() -> i32 {
    return (((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_0)).prefix_inc())
        * (100));
}
pub unsafe fn issue_2(mut ticket: Option<i32>) -> i32 {
    let mut ticket: i32 = ticket.unwrap_or_else(|| unsafe { (unsafe { next_ticket_1() }) });
    return ticket;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_0)) == (0)));
    assert!(((unsafe { issue_2(None,) }) == (100)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_0)) == (1)));
    assert!(((unsafe { issue_2(Some(7),) }) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_0)) == (1)));
    assert!(((unsafe { issue_2(None,) }) == (200)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut next_ticket_calls_0)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const next_ticket_calls_0);
}
