extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Default)]
pub struct Holder {
    pub val: Option<Box<i32>>,
}
pub unsafe fn read_val_0(mut h: *const Holder) -> i32 {
    return (*(*(std::ptr::addr_of!((*h).val).cast_mut()))
        .as_deref_mut()
        .unwrap());
}
pub unsafe fn write_val_1(mut h: *const Holder, mut v: i32) {
    (*(*(std::ptr::addr_of!((*h).val).cast_mut()))
        .as_deref_mut()
        .unwrap()) = v;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: Holder = <Holder>::default();
    h.val = Some(Box::new(10));
    (unsafe {
        let _h: *const Holder = (&mut h as *mut Holder).cast_const();
        let _v: i32 = 42;
        write_val_1(_h, _v)
    });
    return (unsafe {
        let _h: *const Holder = (&mut h as *mut Holder).cast_const();
        read_val_0(_h)
    });
}
