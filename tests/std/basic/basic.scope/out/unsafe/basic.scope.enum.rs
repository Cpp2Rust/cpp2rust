extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub type p1_enumerator_initializer_finds_earlier_enumerator_Step = i32;
pub const p1_enumerator_initializer_finds_earlier_enumerator_Step_first:
    p1_enumerator_initializer_finds_earlier_enumerator_Step = 2;
pub const p1_enumerator_initializer_finds_earlier_enumerator_Step_second:
    p1_enumerator_initializer_finds_earlier_enumerator_Step = 10;
pub type p1_enumerator_initializer_finds_earlier_enumerator_Stride = i32;
pub const p1_enumerator_initializer_finds_earlier_enumerator_Stride_single:
    p1_enumerator_initializer_finds_earlier_enumerator_Stride = 101;
pub unsafe fn test_1() {
    assert!(((p1_enumerator_initializer_finds_earlier_enumerator_Step_first as i32) == (2)));
    assert!(((p1_enumerator_initializer_finds_earlier_enumerator_Step_second as i32) == (10)));
    assert!(((p1_enumerator_initializer_finds_earlier_enumerator_Stride_single as i32) == (101)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_0)) == (100)));
}
pub type p1_scoped_enumerator_does_not_hide_outer_name_Power = i32;
pub const p1_scoped_enumerator_does_not_hide_outer_name_Power_off:
    p1_scoped_enumerator_does_not_hide_outer_name_Power = 3;
pub const p1_scoped_enumerator_does_not_hide_outer_name_Power_on:
    p1_scoped_enumerator_does_not_hide_outer_name_Power = 8;
pub static mut off_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 40 });
pub unsafe fn test_3() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut off_2)) == (40)));
    assert!(((p1_scoped_enumerator_does_not_hide_outer_name_Power_off as i32) == (3)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut off_2)) +=
        (p1_scoped_enumerator_does_not_hide_outer_name_Power_on as i32);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut off_2)) == (48)));
    assert!(((p1_scoped_enumerator_does_not_hide_outer_name_Power_off as i32) == (3)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_0);
    std::cell::LazyCell::force(&*&raw const off_2);
}
