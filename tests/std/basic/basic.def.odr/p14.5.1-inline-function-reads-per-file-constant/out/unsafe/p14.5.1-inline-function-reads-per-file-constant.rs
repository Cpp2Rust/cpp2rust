extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut factor_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub unsafe fn scaled_1(mut v: i32) -> i32 {
    return ((v) * (*std::cell::LazyCell::force_mut(&mut *&raw mut factor_0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut factor_0)) as *const i32)
            != (unsafe { b_factor_address_2() }))
    );
    assert!(((Some(scaled_1)) == (unsafe { b_scaled_address_3() })));
    assert!(((unsafe { scaled_1(2,) }) == (6)));
    assert!(((unsafe { b_scaled_4(2,) }) == (6)));
    return 0;
}
pub unsafe fn b_factor_address_2() -> *const i32 {
    return (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut factor_0)) as *const i32);
}
pub unsafe fn b_scaled_address_3() -> Option<unsafe fn(i32) -> i32> {
    return (Some(scaled_1));
}
pub unsafe fn b_scaled_4(mut v: i32) -> i32 {
    return (unsafe { scaled_1(v) });
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const factor_0);
}
