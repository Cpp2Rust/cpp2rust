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
pub struct Base {
    pub buf: *mut i32,
    pub n: usize,
}
impl Base {
    pub unsafe fn Base(mut b: *mut i32, mut n: usize) -> Self {
        let mut this = Self { buf: b, n: n };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Derived {
    pub base_Base: Base,
}
impl Derived {
    pub unsafe fn begin(&mut self) -> *mut i32 {
        return (*(&mut self.base_Base as *mut Base)).buf;
    }
    pub unsafe fn end(&mut self) -> *mut i32 {
        return (*(&mut self.base_Base as *mut Base))
            .buf
            .offset(((*(&mut self.base_Base as *mut Base)).n) as isize);
    }
    pub unsafe fn Derived(mut _a0: *mut i32, mut _a1: usize) -> Self {
        let mut this = Self {
            base_Base: Base::Base({ _a0 }, { _a1 }),
        };
        this
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut d: Derived = Derived::Derived({ arr.as_mut_ptr() }, { 3_usize });
    assert!(((d.base_Base.n) == (3_usize)));
    assert!(((*d.base_Base.buf.offset((1) as isize)) == (2)));
    assert!(((*(unsafe { Derived::begin(&mut d,) })) == (1)));
    assert!(
        (((((unsafe { Derived::end(&mut d,) }) as usize
            - (unsafe { Derived::begin(&mut d,) }) as usize)
            / ::std::mem::size_of::<i32>()) as i64)
            == (3_i64))
    );
    return 0;
}
