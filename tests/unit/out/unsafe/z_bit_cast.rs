extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn decay_cast_0(mut a1: *mut u32) {}
pub unsafe fn bit_cast_1(mut p: *const ::libc::c_void) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    (unsafe { decay_cast_0(a1.as_mut_ptr()) });
    (unsafe { decay_cast_0((&mut a1[(0) as usize] as *mut u32)) });
    (unsafe { bit_cast_1((a1.as_mut_ptr() as *const u32 as *const ::libc::c_void)) });
    (unsafe {
        bit_cast_1(((&mut a1[(0) as usize] as *mut u32) as *const u32 as *const ::libc::c_void))
    });
    (unsafe {
        bit_cast_1(((&mut a1 as *mut [u32; 3]) as *const [u32; 3] as *const ::libc::c_void))
    });
    let mut ptr: *mut ::libc::c_void = (a1.as_mut_ptr() as *mut u32 as *mut ::libc::c_void);
    assert!(((ptr) == (a1.as_mut_ptr() as *mut u32 as *mut ::libc::c_void)));
    assert!(((*(ptr as *mut u32).offset((0) as isize)) == (a1[(0) as usize])));
    return 0;
}
