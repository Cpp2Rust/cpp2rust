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
pub struct pair {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [pair; 3] = [<pair>::default(); 3];
    arr[(0) as usize].x = 10;
    arr[(1) as usize].x = 20;
    arr[(2) as usize].x = 30;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union anon_0 {
        pub p: *mut pair,
        pub bits: u64,
    }
    impl Default for anon_0 {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    };
    let mut u: anon_0 = <anon_0>::default();
    u.p = (&mut arr[(1) as usize] as *mut pair);
    let mut q: *mut pair = u.p;
    assert!((((((*q).x) == (20)) as i32) != 0));
    assert!(((((q) == (&mut arr[(1) as usize] as *mut pair)) as i32) != 0));
    u.bits = ((u.bits as u64).wrapping_add((::std::mem::size_of::<pair>() as u64))) as u64;
    assert!((((((*u.p).x) == (30)) as i32) != 0));
    assert!(((((u.p) == (&mut arr[(2) as usize] as *mut pair)) as i32) != 0));
    u.bits = ((u.bits as u64)
        .wrapping_sub(((2_usize).wrapping_mul((::std::mem::size_of::<pair>() as usize)) as u64)))
        as u64;
    assert!((((((*u.p).x) == (10)) as i32) != 0));
    assert!(((((u.p) == (&mut arr[(0) as usize] as *mut pair)) as i32) != 0));
    return 0;
}
