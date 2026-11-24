extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut a: [u32; 3] = [10_u32, 20_u32, 30_u32];
    let mut i: i32 = 0;
    match ({ 5_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
        }
    };
    (unsafe {
        libc::printf(
            c"void %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut sum: u32 = (match ({ 2_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (((*__tmp_3) as u32).wrapping_add((__tmp_2 as u32))) as u32;
            (*__tmp_3).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"compound %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (sum as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut assigned: u32 = (match ({ 99_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_4, __tmp_5) => {
            (*__tmp_5) = __tmp_4;
            (*__tmp_5).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"simple %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (assigned as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
}
pub unsafe fn test_1() {
    let mut a: [i16; 3] = [10_i16, 20_i16, 30_i16];
    let mut i: i32 = 0;
    match ({ 5 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as i32) + (__tmp_6 as i32)) as i16;
        }
    };
    (unsafe {
        libc::printf(
            c"void %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut sum: i16 = (match ({ 2 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as i32) + (__tmp_8 as i32)) as i16;
            (*__tmp_9).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"compound %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (sum as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut assigned: i16 = (match ({ 99_i16 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_10, __tmp_11) => {
            (*__tmp_11) = __tmp_10;
            (*__tmp_11).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"simple %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (assigned as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
}
pub unsafe fn test_2() {
    let mut a: [i32; 3] = [10, 20, 30];
    let mut i: i32 = 0;
    a[(i.postfix_inc()) as usize] += 5;
    (unsafe {
        libc::printf(
            c"void %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (a[(0) as usize]),
            (a[(1) as usize]),
        )
    });
    i = 0;
    let mut sum: i32 = (match ({ 2 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_12, __tmp_13) => {
            (*__tmp_13) = (((*__tmp_13) as i32) + (__tmp_12 as i32)) as i32;
            (*__tmp_13).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"compound %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (sum),
            (a[(0) as usize]),
            (a[(1) as usize]),
        )
    });
    i = 0;
    let mut assigned: i32 = (match ({ 99 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_14, __tmp_15) => {
            (*__tmp_15) = __tmp_14;
            (*__tmp_15).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"simple %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (assigned),
            (a[(0) as usize]),
            (a[(1) as usize]),
        )
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
