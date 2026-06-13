extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut v: Vec<core::ffi::c_char>) -> Vec<core::ffi::c_char> {
    return {
        let mut __tmp2 = v.clone();
        __tmp2.pop();
        let __from = c" str".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
}
pub unsafe fn fn2_1(v: *const Vec<core::ffi::c_char>) -> *const Vec<core::ffi::c_char> {
    return v;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    printf(c"%s\n".as_ptr() as *const i8, c"fprintf stdout".as_ptr());
    printf(c"%d %u %ld\n".as_ptr() as *const i8, 1, 2_u32, 3_i64);
    printf(c"hello world".as_ptr() as *const i8);
    let mut in_: *mut ::libc::FILE = libcc2rs::stdin_unsafe();
    assert!(!((in_).is_null()));
    printf(c"%s\n".as_ptr() as *const i8, c"printf".as_ptr());
    printf(c"hello world".as_ptr() as *const i8);
    let mut s: Vec<core::ffi::c_char> = {
        let s = c"a string".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    printf(c"%s\n".as_ptr() as *const i8, s.as_mut_ptr());
    printf(
        c"%s\n".as_ptr() as *const i8,
        (unsafe {
            let _v: Vec<core::ffi::c_char> = {
                let s = c"foo".as_ptr();
                std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                    .to_vec()
            };
            fn_0(_v)
        })
        .as_ptr(),
    );
    printf(
        c"%s\n".as_ptr() as *const i8,
        (*(unsafe {
            let _v: *const Vec<core::ffi::c_char> = &s as *const Vec<core::ffi::c_char>;
            fn2_1(_v)
        }))
        .as_ptr(),
    );
    return 0;
}
