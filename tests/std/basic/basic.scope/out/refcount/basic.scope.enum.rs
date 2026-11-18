extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_0: Value<i32> = Rc::new(RefCell::new(100));
);
pub type p1_enumerator_initializer_finds_earlier_enumerator_Step = i32;
pub const p1_enumerator_initializer_finds_earlier_enumerator_Step_first:
    p1_enumerator_initializer_finds_earlier_enumerator_Step = 2;
pub const p1_enumerator_initializer_finds_earlier_enumerator_Step_second:
    p1_enumerator_initializer_finds_earlier_enumerator_Step = 10;
pub type p1_enumerator_initializer_finds_earlier_enumerator_Stride = i32;
pub const p1_enumerator_initializer_finds_earlier_enumerator_Stride_single:
    p1_enumerator_initializer_finds_earlier_enumerator_Stride = 101;
pub fn test_1() {
    assert!(((p1_enumerator_initializer_finds_earlier_enumerator_Step_first as i32) == 2));
    assert!(((p1_enumerator_initializer_finds_earlier_enumerator_Step_second as i32) == 10));
    assert!(((p1_enumerator_initializer_finds_earlier_enumerator_Stride_single as i32) == 101));
    assert!((first_0.with(|rc| *rc.borrow()) == 100));
}
pub type p1_scoped_enumerator_does_not_hide_outer_name_Power = i32;
pub const p1_scoped_enumerator_does_not_hide_outer_name_Power_off:
    p1_scoped_enumerator_does_not_hide_outer_name_Power = 3;
pub const p1_scoped_enumerator_does_not_hide_outer_name_Power_on:
    p1_scoped_enumerator_does_not_hide_outer_name_Power = 8;
thread_local!(
    pub static off_2: Value<i32> = Rc::new(RefCell::new(40));
);
pub fn test_3() {
    assert!((off_2.with(|rc| *rc.borrow()) == 40));
    assert!(((p1_scoped_enumerator_does_not_hide_outer_name_Power_off as i32) == 3));
    off_2.with(|rc| {
        *rc.borrow_mut() += (p1_scoped_enumerator_does_not_hide_outer_name_Power_on as i32)
    });
    assert!((off_2.with(|rc| *rc.borrow()) == 48));
    assert!(((p1_scoped_enumerator_does_not_hide_outer_name_Power_off as i32) == 3));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = first_0.with(|_| ());
    let _ = off_2.with(|_| ());
}
