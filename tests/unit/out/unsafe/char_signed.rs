extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn to_int_0(mut c: libc::c_char) -> i32 {
    return (c as i32);
}
pub unsafe fn is_negative_1(mut s: *const libc::c_char) -> bool {
    return (((*s.offset((0) as isize)) as i32) < (0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: libc::c_char = (-56 as libc::c_char);
    let mut widened: i32 = (c as i32);
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (widened),
            (unsafe { to_int_0((b'\xff' as libc::c_char)) }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (((c as i32) < (0)) as i32),
        )
    });
    let mut lit: [libc::c_char; 4] = std::mem::transmute(*b"\xe9t\xe9\0");
    let mut ulit: [u8; 4] = std::mem::transmute(*b"\xe9t\xe9\0");
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (lit[(0) as usize] as i32),
            (ulit[(0) as usize] as i32),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            ((unsafe { is_negative_1((lit.as_mut_ptr()).cast_const()) }) as i32),
        )
    });
    let mut u8s: *const libc::c_char = c"\xc3\xa9".as_ptr();
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            ((*u8s.offset((0) as isize)) as i32),
            ((*u8s.offset((1) as isize)) as i32),
        )
    });
    let mut p: *const libc::c_char = c"\x80".as_ptr();
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            ((*p.offset((0) as isize)) as i32),
            (((*p.offset((0) as isize)) as u8) as i32),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (((libc::strcmp(c"\x80".as_ptr(), c"a".as_ptr())) > (0)) as i32),
        )
    });
    let mut s: Vec<libc::c_char> = {
        let s = c"\xfe!".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    (unsafe {
        libc::printf(
            c"%d %d\n".as_ptr() as *const libc::c_char,
            (s[(0_usize)] as i32),
            (s[(1_usize)] as i32),
        )
    });
    let mut sum: libc::c_char = (((c as i32) + (c as i32)) as libc::c_char);
    (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (sum as i32)) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
