extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive()]
pub struct S {
    pub v: Vec<i32>,
    pub n: [i32; 2],
}
impl S {
    pub unsafe fn S(mut x: i32) -> Self {
        let mut this = Self {
            v: vec![x; (x as usize) as usize],
            n: [x, ((x) + (1))],
        };
        this
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            v: Default::default(),
            n: [0_i32; 2],
        }
    }
}
pub unsafe fn sum_0(s: *const S) -> i32 {
    return ((((*s).v.len() as i32) + ((*s).n[(0) as usize])) + ((*s).n[(1) as usize]));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::S({ 2 });
    assert!(((unsafe { sum_0(&s as *const S,) }) == (7)));
    assert!(((unsafe { shuffle_1(3,) }) == (10)));
    return 0;
}
impl S {
    pub unsafe fn S_pmutS(_a0: *mut S) -> Self {
        let mut this = Self {
            v: std::mem::take(&mut (*_a0).v),
            n: (*_a0).n,
        };
        this
    }
    pub unsafe fn operator_assign_pmutS(&mut self, _a0: *mut S) -> *mut S {
        self.v = std::mem::take(&mut (*_a0).v);
        self.n = ((*_a0).n).clone();
        return &mut (*(self as *mut S)) as *mut S;
    }
}
pub unsafe fn shuffle_1(mut x: i32) -> i32 {
    let mut a: S = S::S({ x });
    let mut b: S = S::S_pmutS({ &mut a as *mut S });
    assert!(a.v.is_empty());
    let mut c: S = S::S({ 1 });
    (unsafe { S::operator_assign_pmutS(&mut c, &mut b as *mut S) });
    assert!(b.v.is_empty());
    return (unsafe { sum_0(&c as *const S) });
}
