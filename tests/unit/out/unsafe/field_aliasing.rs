extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct In {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg)]
pub struct S {
    pub in_: In,
    pub total: i32,
    pub n: i32,
    pub arr: [i32; 4],
}
impl Default for S {
    fn default() -> Self {
        S {
            in_: <In>::default(),
            total: 0_i32,
            n: 0_i32,
            arr: [0_i32; 4],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Node {
    pub x: i32,
    pub self_: *mut Node,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut S = (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<S>()) as *mut S);
    assert!((((!((p).is_null())) as i32) != 0));
    let mut q: *mut S = p;
    (*p).in_.x = 1;
    (*p).in_.y = 2;
    (*p).total = (((*q).in_.x) + ((*q).in_.y));
    assert!((((((*q).total) == (3)) as i32) != 0));
    let mut ip: *mut In = (&mut (*p).in_ as *mut In);
    (*ip).x = (((*p).total) + (1));
    assert!(
        ((((((((*q).in_.x) == (4)) as i32) != 0) && (((((*q).in_.y) == (2)) as i32) != 0)) as i32)
            != 0)
    );
    (*p).arr[((*p).n) as usize] = (*p).total;
    (*p).n += 1;
    (*p).arr[((*p).n) as usize] = (*q).in_.x;
    assert!(
        (((((((((((*q).arr[(0) as usize]) == (3)) as i32) != 0)
            && (((((*q).arr[(1) as usize]) == (4)) as i32) != 0)) as i32)
            != 0)
            && (((((*q).n) == (1)) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe((p as *mut ::libc::c_void));
    let mut s: Node = <Node>::default();
    s.x = 1;
    s.self_ = (&mut s as *mut Node);
    (*s.self_).x = ((s.x) + (1));
    assert!(((((s.x) == (2)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
