extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Overload = u32;
pub const Overload_kMutableOverload: Overload = 1;
pub const Overload_kConstOverload: Overload = 2;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn f(&mut self) -> Overload {
        return Overload_kMutableOverload;
    }
    pub unsafe fn f_const(&self) -> Overload {
        return Overload_kConstOverload;
    }
    pub unsafe fn value_ref(&mut self) -> *mut i32 {
        return &mut self.v;
    }
    pub unsafe fn value_ref_const(&self) -> *const i32 {
        return &self.v;
    }
}
pub unsafe fn g_0(_a0: *mut S) -> Overload {
    return Overload_kMutableOverload;
}
pub unsafe fn g_1(_a0: *const S) -> Overload {
    return Overload_kConstOverload;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 7 };
    assert!((((unsafe { S::f(&mut s,) }) as i32) == (Overload_kMutableOverload as i32)));
    assert!((((unsafe { S::f_const(&s,) }) as i32) == (Overload_kConstOverload as i32)));
    assert!((((unsafe { g_0(&mut s,) }) as i32) == (Overload_kMutableOverload as i32)));
    assert!((((unsafe { g_1(&s,) }) as i32) == (Overload_kConstOverload as i32)));
    (*(unsafe { S::value_ref(&mut s) })) = 9;
    assert!(((s.v) == (9)));
    assert!(((*(unsafe { S::value_ref_const(&s,) })) == (9)));
    let cs: *const S = &s;
    assert!((((unsafe { S::f_const(&(*cs),) }) as i32) == (Overload_kConstOverload as i32)));
    assert!((((*cs).v) == (9)));
    let mut p: *mut S = (&mut s as *mut S);
    (*p).v = 11;
    assert!(((s.v) == (11)));
    assert!((((unsafe { S::f(&mut (*p),) }) as i32) == (Overload_kMutableOverload as i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
