extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Base {
    pub buf: [libc::c_char; 8],
}
impl Base {
    pub unsafe fn fill(&mut self, mut c: libc::c_char, mut n: i32) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (n)) {
            self.buf[(i) as usize] = c;
            i.postfix_inc();
        }
    }
}
impl Default for Base {
    fn default() -> Self {
        Base {
            buf: [(0 as libc::c_char); 8],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Derived {
    pub __base: Base,
}
impl Derived {
    pub unsafe fn run(&mut self) -> bool {
        (unsafe { Base::fill(self, ('x' as libc::c_char), 3) });
        return (((self as *mut Base).buf[(2) as usize] as i32) == (('x' as libc::c_char) as i32));
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: Derived = <Derived>::default();
    assert!((unsafe { Derived::run(&mut d,) }));
    (unsafe { Base::fill(&mut (d as Base), ('y' as libc::c_char), 1) });
    assert!((((d as Base).buf[(0) as usize] as i32) == (('y' as libc::c_char) as i32)));
    assert!((((d as Base).buf[(1) as usize] as i32) == (('x' as libc::c_char) as i32)));
    return 0;
}
