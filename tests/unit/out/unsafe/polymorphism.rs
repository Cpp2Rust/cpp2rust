extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait Animal {
    unsafe fn bark(&self) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Dog {}
unsafe impl Animal for Dog {
    unsafe fn bark(&self) -> bool {
        return true;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Cat {}
impl Cat {
    unsafe fn meow(&self) -> bool {
        return true;
    }
}
unsafe impl Animal for Cat {
    unsafe fn bark(&self) -> bool {
        return false;
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dog: Dog = <Dog>::default();
    let mut animal: *mut dyn Animal = (&mut dog as *mut Dog);
    let mut eat1: bool = (unsafe { (*animal.cast_const()).bark() });
    let mut cat: Cat = <Cat>::default();
    animal = (&mut cat as *mut Cat);
    let mut eat2: bool = (unsafe { (*animal.cast_const()).bark() });
    return (((eat1) && (!(eat2) as bool)) as i32);
}
