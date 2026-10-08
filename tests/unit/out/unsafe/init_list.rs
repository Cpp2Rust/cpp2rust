extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut list: Vec<i32>) {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Holder {
    pub p: Pair,
}
impl Holder {
    pub unsafe fn operator_assign_1(&mut self, o: *mut Pair) -> *mut Holder {
        self.p = (*o);
        return &mut (*(self as *mut Holder));
    }
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            p: Pair { a: 0, b: 0 },
        }
    }
}
pub unsafe fn sum_1(p: *const Pair) -> i32 {
    return (((*p).a) + ((*p).b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = 0_i32;
    let mut carr1: [i32; 2] = [1, 2];
    let mut carr2: [i32; 3] = [1, 0_i32, 0_i32];
    let mut arr: Vec<i32> = vec![1, 2, 3];
    let mut vec_: Vec<i32> = vec![1, 2, 3];
    (unsafe { f_0(vec![1, 2, 3, 4]) });
    let mut p: Pair = Pair { a: 1, b: 2 };
    let mut h: Holder = <Holder>::default();
    (unsafe { Holder::operator_assign_1(&mut h, &mut p) });
    assert!(((h.p.a) == (1)) && ((h.p.b) == (2)));
    assert!(((unsafe { sum_1(&p,) }) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
