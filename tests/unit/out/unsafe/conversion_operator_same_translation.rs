extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn to_libcc_char(&self) -> libc::c_char {
        return (self.v as libc::c_char);
    }
    pub unsafe fn to_u8(&self) -> u8 {
        return (((self.v) + (1)) as u8);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct T {
    pub v: i32,
}
impl T {
    pub unsafe fn to_i64_1(&self) -> i64 {
        return (self.v as i64);
    }
    pub unsafe fn to_i64_2(&self) -> i64 {
        return (((self.v) + (1)) as i64);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 65 };
    let mut a: libc::c_char = (unsafe { S::to_libcc_char(&s) });
    let mut b: u8 = (unsafe { S::to_u8(&s) });
    assert!(((a as i32) == (('A' as libc::c_char) as i32)));
    assert!(((b as i32) == ((66 as u8) as i32)));
    assert!(
        ((((unsafe { S::to_libcc_char(&s,) }) as i32) + ((unsafe { S::to_u8(&s,) }) as i32))
            == (131))
    );
    let mut t: T = T { v: 3 };
    let mut c: i64 = (unsafe { T::to_i64_1(&t) });
    let mut d: i64 = (unsafe { T::to_i64_2(&t) });
    assert!(((c) == (3_i64)));
    assert!(((d) == (4_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
