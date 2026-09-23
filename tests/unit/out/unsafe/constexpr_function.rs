extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn runtime_only_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn first_1(mut p: *const i32) -> i32 {
    return (*p);
}
pub unsafe fn scaled_2(mut x: i32) -> i32 {
    if ((x) < (0)) {
        return (unsafe { runtime_only_0(-x) });
    }
    return x;
}
pub unsafe fn half_3(mut x: f64) -> f64 {
    return ((x) / (2.0E+0));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Flag {
    pub v: i32,
}
impl Flag {
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.v) != (0));
    }
}
pub unsafe fn use_4(mut f: Flag) -> i32 {
    assert!((unsafe { Flag::to_bool(&f,) }));
    return f.v;
}
pub unsafe fn checked_5(mut x: i32) -> i32 {
    assert!(((x) > (0)));
    return ((x) + (1));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct P {
    pub v: i32,
}
impl P {
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i32; 2] = [7, 8];
    assert!(((unsafe { first_1((arr.as_mut_ptr()).cast_const(),) }) == (7)));
    assert!(((unsafe { first_1((arr.as_mut_ptr().offset((1) as isize)).cast_const(),) }) == (8)));
    assert!(((unsafe { scaled_2(3,) }) == (3)));
    assert!(((unsafe { scaled_2(-3_i32,) }) == (6)));
    assert!(((unsafe { half_3(5.0E+0,) }) == (2.5E+0)));
    assert!(((unsafe { checked_5(1,) }) == (2)));
    let c: i32 = (unsafe { checked_5(4) });
    assert!(((c) == (5)));
    assert!(((unsafe { use_4(Flag { v: 2 },) }) == (2)));
    let u: i32 = (unsafe { use_4(Flag { v: 3 }) });
    assert!(((u) == (3)));
    let mut ptr: *mut i32 = arr.as_mut_ptr();
    assert!(!(ptr).is_null());
    let mut p: P = P { v: 9 };
    assert!(((unsafe { P::get(&p,) }) == (9)));
    let k: i32 = (unsafe { scaled_2(4) });
    assert!(((k) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
