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
pub struct Counter_Impl_ {
    pub n: i32,
}
impl Counter_Impl_ {
    pub unsafe fn inc(&mut self) -> *mut Impl {
        self.n.prefix_inc();
        return &mut ((*self) as *mut Impl) as *mut Impl;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Impl {
    pub base_Counter_Impl_: Counter_Impl_,
}
impl Impl {
    pub unsafe fn twice(&mut self) -> i32 {
        return ((self.base_Counter_Impl_.n) * (2));
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i: Impl = <Impl>::default();
    (unsafe {
        Counter_Impl_::inc(
            &mut (*(unsafe { Counter_Impl_::inc(&mut i.base_Counter_Impl_) })).base_Counter_Impl_,
        )
    });
    assert!(((unsafe { Impl::twice(&mut i,) }) == (4)));
    assert!(
        ((unsafe {
            Impl::twice(&mut (*(unsafe { Counter_Impl_::inc(&mut i.base_Counter_Impl_) })))
        }) == (6))
    );
    return 0;
}
