extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_not(&self) -> bool {
        let this = self as *const S;
        return (((*this).v) == (0));
    }
    pub unsafe fn operator_and(&self, o: *const S) -> bool {
        let this = self as *const S;
        return (((*this).v) != (0)) && (((*o).v) != (0));
    }
    pub unsafe fn operator_or(&self, o: *const S) -> bool {
        let this = self as *const S;
        return (((*this).v) != (0)) || (((*o).v) != (0));
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: S = S { v: 1 };
    let mut f: S = S { v: 0 };
    assert!((unsafe { S::operator_not(&f,) }));
    assert!(!(unsafe { S::operator_not(&t,) }));
    assert!(
        (unsafe {
            let _o: *const S = &t as *const S;
            S::operator_and(&t, _o)
        })
    );
    assert!(!(unsafe { S::operator_and(&t, &f as *const S,) }));
    assert!((unsafe { S::operator_or(&t, &f as *const S,) }));
    assert!(
        !(unsafe {
            let _o: *const S = &f as *const S;
            S::operator_or(&f, _o)
        })
    );
    return 0;
}
