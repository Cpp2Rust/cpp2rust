extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vtable {
    pub create: Option<unsafe fn(i32) -> *mut ::libc::c_void>,
    pub get: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    pub destroy: Option<unsafe fn(*mut ::libc::c_void)>,
}
impl Default for Vtable {
    fn default() -> Self {
        Vtable {
            create: None,
            get: None,
            destroy: None,
        }
    }
}
pub static mut storage_0: i32 = unsafe { 0_i32 };
pub unsafe fn int_create_1(mut val: i32) -> *mut ::libc::c_void {
    storage_0 = val;
    return ((&raw mut storage_0 as *mut i32) as *mut i32 as *mut ::libc::c_void);
}
pub unsafe fn int_get_2(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn int_destroy_3(mut p: *mut ::libc::c_void) {
    (*(p as *mut i32)) = 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vt: Vtable = Vtable {
        create: Some(int_create_1),
        get: Some(int_get_2),
        destroy: Some(int_destroy_3),
    };
    assert!(!((vt.create).is_none()));
    assert!(!((vt.get).is_none()));
    assert!(!((vt.destroy).is_none()));
    let mut obj: *mut ::libc::c_void = (unsafe { (vt.create).unwrap()(42) });
    assert!(
        ((unsafe {
            let _arg0: *mut ::libc::c_void = obj;
            (vt.get).unwrap()(_arg0)
        }) == (42))
    );
    (unsafe {
        let _arg0: *mut ::libc::c_void = obj;
        (vt.destroy).unwrap()(_arg0)
    });
    assert!(((storage_0) == (0)));
    (vt.get) = None;
    assert!((vt.get).is_none());
    return 0;
}
