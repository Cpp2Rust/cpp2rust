extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut v: Vec<u8>) -> Vec<u8> {
    return {
        let mut __tmp2 = v.clone();
        __tmp2.pop();
        let __from = b" str\0".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
}
pub unsafe fn fn2_1(v: *const Vec<u8>) -> *const Vec<u8> {
    return v;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    printf(
        b"%s\n\0".as_ptr() as *const i8,
        b"fprintf stdout\0".as_ptr(),
    );
    printf(b"%d %u %ld\n\0".as_ptr() as *const i8, 1, 2_u32, 3_i64);
    printf(b"hello world\0".as_ptr() as *const i8);
    let mut in_: *mut ::std::fs::File = libcc2rs::cin_unsafe();
    assert!(!((in_).is_null()));
    printf(b"%s\n\0".as_ptr() as *const i8, b"printf\0".as_ptr());
    printf(b"hello world\0".as_ptr() as *const i8);
    let mut s: Vec<u8> = {
        let s = b"a string\0".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    printf(b"%s\n\0".as_ptr() as *const i8, s.as_mut_ptr());
    printf(
        b"%s\n\0".as_ptr() as *const i8,
        (unsafe {
            let _v: Vec<u8> = {
                let s = b"foo\0".as_ptr();
                std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                    .to_vec()
            };
            fn_0(_v)
        })
        .as_ptr(),
    );
    printf(
        b"%s\n\0".as_ptr() as *const i8,
        (*(unsafe {
            let _v: *const Vec<u8> = &s as *const Vec<u8>;
            fn2_1(_v)
        }))
        .as_ptr(),
    );
    return 0;
}
