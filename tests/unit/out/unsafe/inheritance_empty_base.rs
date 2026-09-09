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
pub struct Tag {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct View {
    pub base_Tag: Tag,
    pub i: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Base {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Derived {
    pub base_Base: Base,
}
pub unsafe fn as_base_0(mut d: *mut Derived) -> *mut Base {
    return (&mut (*d).base_Base as *mut Base);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: View = <View>::default();
    v.i = 5;
    assert!(((v.i) == (5)));
    let mut d: Derived = <Derived>::default();
    let mut b: *mut Base = (unsafe { as_base_0((&mut d as *mut Derived)) });
    assert!(((b) == (&mut (*(&mut d as *mut Derived)).base_Base as *mut Base)));
    return 0;
}
