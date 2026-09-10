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
pub struct Inner {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub data: [i32; 3],
    pub inner: Inner,
}
impl Default for S {
    fn default() -> Self {
        S {
            data: [0_i32; 3],
            inner: <Inner>::default(),
        }
    }
}
pub unsafe fn operator_deref_0(s: *mut S) -> *mut Inner {
    return &mut (*s).inner as *mut Inner;
}
pub unsafe fn operator_addr_1(s: *mut S) -> *mut i32 {
    return (&mut (*s).data[(0) as usize] as *mut i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S {
        data: [1, 2, 3],
        inner: Inner { x: 9 },
    };
    assert!(
        (((*(unsafe {
            let _s: *mut S = &mut s as *mut S;
            operator_deref_0(_s)
        }))
        .x) == (9))
    );
    (*(unsafe {
        let _s: *mut S = &mut s as *mut S;
        operator_deref_0(_s)
    }))
    .x = 10;
    assert!(((s.inner.x) == (10)));
    let mut p: *mut i32 = (unsafe {
        let _s: *mut S = &mut s as *mut S;
        operator_addr_1(_s)
    });
    assert!(((*p) == (1)));
    (*p) = 5;
    assert!(((s.data[(0) as usize]) == (5)));
    return 0;
}
