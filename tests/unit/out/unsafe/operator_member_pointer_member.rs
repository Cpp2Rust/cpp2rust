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
impl S {
    pub unsafe fn operator_index_i32(&mut self, mut i: i32) -> *mut i32 {
        return &mut self.data[(i) as usize] as *mut i32;
    }
    pub unsafe fn operator_index_i32_const(&self, mut i: i32) -> *const i32 {
        return &self.data[(i) as usize] as *const i32;
    }
    pub unsafe fn operator_deref(&mut self) -> *mut Inner {
        return &mut self.inner as *mut Inner;
    }
    pub unsafe fn operator_arrow(&mut self) -> *mut Inner {
        return (&mut self.inner as *mut Inner);
    }
    pub unsafe fn operator_addr(&mut self) -> *mut i32 {
        return (&mut self.data[(0) as usize] as *mut i32);
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            data: [0_i32; 3],
            inner: <Inner>::default(),
        }
    }
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
    assert!(((*(unsafe { S::operator_index_i32(&mut s, 1,) })) == (2)));
    (*(unsafe { S::operator_index_i32(&mut s, 1) })) = 20;
    assert!(((*(unsafe { S::operator_index_i32(&mut s, 1,) })) == (20)));
    let cs: *const S = &s as *const S;
    assert!(((*(unsafe { S::operator_index_i32_const(&(*cs), 2,) })) == (3)));
    assert!((((*(unsafe { S::operator_deref(&mut s,) })).x) == (9)));
    (*(unsafe { S::operator_deref(&mut s) })).x = 10;
    assert!((((*(unsafe { S::operator_arrow(&mut s,) })).x) == (10)));
    (*(unsafe { S::operator_arrow(&mut s) })).x = 11;
    assert!(((s.inner.x) == (11)));
    let mut p: *mut i32 = (unsafe { S::operator_addr(&mut s) });
    assert!(((*p) == (1)));
    (*p) = 5;
    assert!(((s.data[(0) as usize]) == (5)));
    return 0;
}
