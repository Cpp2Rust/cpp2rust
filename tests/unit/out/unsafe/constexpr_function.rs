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
    let mut p: P = P { v: 9 };
    assert!(((unsafe { P::get(&p,) }) == (9)));
    let k: i32 = (unsafe { scaled_2(4) });
    assert!(((k) == (4)));
    return 0;
}
