extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: Option<Box<i32>> = Some(Box::new(10));
    (*p.as_deref_mut().unwrap()) = (((*p.as_deref_mut().unwrap()) as i32) + 5) as i32;
    (*p.as_deref_mut().unwrap()) = (((*p.as_deref_mut().unwrap()) as i32) - 3) as i32;
    (*p.as_deref_mut().unwrap()) = (((*p.as_deref_mut().unwrap()) as i32) * 2) as i32;
    let mut q: Option<Box<i32>> = Some(Box::new(1));
    let mut sum: i32 = ((*p.as_deref_mut().unwrap()) + (*q.as_deref_mut().unwrap()));
    return sum;
}
