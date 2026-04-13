extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(
    mut fn_: Rc<dyn Fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
    mut p: *mut ::libc::c_void,
    mut size: u64,
    mut nmemb: u64,
    mut f: *mut ::std::fs::File,
) -> u64 {
    return (unsafe {
        let _arg0: *mut ::libc::c_void = p;
        let _arg1: u64 = size;
        let _arg2: u64 = nmemb;
        let _arg3: *mut ::std::fs::File = f;
        fn_(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Rc<dyn Fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
        (Rc::new(|a0, a1, a2, a3| unsafe { fread_1(a0, a1, a2, a3) })
            as Rc<dyn Fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>);
    (unsafe {
        let _fn: Rc<dyn Fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
            Rc::new(|a0, a1, a2, a3| unsafe { fread_1(a0, a1, a2, a3) });
        let _p: *mut ::libc::c_void = Default::default();
        let _size: u64 = 0_u64;
        let _nmemb: u64 = 0_u64;
        let _f: *mut ::std::fs::File = Default::default();
        foo_0(_fn, _p, _size, _nmemb, _f)
    });
    assert!(((fn_) != (Default::default())));
    return 0;
}
