extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone)]
pub struct record {
    pub code: u16,
    pub pad: [u8; 14],
}
#[derive(Copy, Clone)]
pub struct inner_anon_0 {
    pub h: record,
    pub raw_: [u8; 128],
}
#[derive(Copy, Clone, Default)]
pub struct inner {
    pub view: inner_anon_0,
}
#[derive(Copy, Clone, Default)]
pub struct Outer_anon_0 {
    pub h: record,
    pub nested: inner,
}
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub kind: i32,
    pub level: i32,
    pub variant: i32,
    pub len: u32,
    pub body: Outer_anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ex: Outer = <Outer>::default();
    {
        let byte_0 = ((&mut ex as *mut Outer) as *mut Outer as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Outer>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut ex as *mut Outer) as *mut Outer as *mut ::libc::c_void)
    };
    ex.kind = 2;
    ex.level = 1;
    ex.variant = 6;
    ex.len = (::std::mem::size_of::<record>() as u64 as u32);
    ex.body.h.code = 2_u16;
    ex.body.h.pad[(0) as usize] = (('X' as i32) as u8);
    assert!(((ex.body.h.code as i32) == (2)));
    assert!(((ex.body.h.pad[(0) as usize] as i32) == ('X' as i32)));
    assert!(((ex.body.nested.view.h.code as i32) == (2)));
    return 0;
}
