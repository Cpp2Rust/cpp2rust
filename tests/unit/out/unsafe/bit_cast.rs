#![feature(rustc_private)]
extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::prepostfix::*;
pub unsafe fn decay_cast(mut a1: *mut u32) {}
pub unsafe fn bit_cast(mut p: *const ::libc::c_void) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
pub unsafe fn main_0() -> i32 {
    let mut a1: [u32; 3] = [(1_i32 as u32), (2_i32 as u32), (3_i32 as u32)];
    decay_cast(a1.as_mut_ptr());
    decay_cast(&mut a1[0_i32 as usize]);
    bit_cast((a1.as_mut_ptr() as *const u32 as *const ::libc::c_void));
    bit_cast((&a1[0_i32 as usize] as *const u32 as *const ::libc::c_void));
    bit_cast((&a1 as *const [u32; 3] as *const ::libc::c_void));
    let mut ptr: *mut ::libc::c_void = (a1.as_mut_ptr() as *mut u32 as *mut ::libc::c_void);
    (if ((!(ptr == (a1.as_mut_ptr() as *mut u32 as *mut ::libc::c_void)) as i64) != 0) {
        panic!(
            "assertion failed: {}",
            ::std::ffi::CStr::from_ptr(b"ptr == a1\0" as *const u8 as *const i8)
                .to_str()
                .unwrap()
                .to_owned()
        )
    } else {
    });
    (if ((!((*(ptr as *mut u32).offset(0_i32 as isize)) == a1[0_i32 as usize]) as i64) != 0) {
        panic!(
            "assertion failed: {}",
            ::std::ffi::CStr::from_ptr(
                b"((unsigned int*)ptr)[0] == a1[0]\0" as *const u8 as *const i8
            )
            .to_str()
            .unwrap()
            .to_owned()
        )
    } else {
    });
    return 0_i32;
}
