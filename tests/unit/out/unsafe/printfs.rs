extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut v: Vec<libc::c_char>) -> Vec<libc::c_char> {
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
pub unsafe fn fn2_1(v: *const Vec<libc::c_char>) -> *const Vec<libc::c_char> {
    return v;
}
pub unsafe fn log_to_2(
    mut out: *mut ::libc::FILE,
    mut fmt: *const libc::c_char,
    mut args_2: *const libc::c_char,
    mut args_3: i32,
) {
    (unsafe {
        libc::fprintf(
            out as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (args_2),
            (args_3),
        )
    });
}
pub unsafe fn log_to_3(mut out: *mut ::libc::FILE, mut fmt: *const libc::c_char, mut args: i32) {
    (unsafe { libc::fprintf(out as *mut ::libc::FILE, fmt as *const libc::c_char, (args)) });
}
pub unsafe fn log_fmt_4(mut fmt: *const libc::c_char, mut v: i32) {
    (unsafe { libc::printf(fmt as *const libc::c_char, (v)) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe {
        libc::fprintf(
            libcc2rs::stdout_unsafe() as *mut ::libc::FILE,
            c"%s\n".as_ptr() as *const libc::c_char,
            (c"fprintf stdout".as_ptr()),
        )
    });
    (unsafe {
        libc::fprintf(
            libcc2rs::stdout_unsafe() as *mut ::libc::FILE,
            c"%d %u %ld\n".as_ptr() as *const libc::c_char,
            (1),
            (2_u32),
            (3_i64),
        )
    });
    (unsafe {
        libc::fprintf(
            libcc2rs::stdout_unsafe() as *mut ::libc::FILE,
            c"hello world".as_ptr() as *const libc::c_char,
        )
    });
    let mut in_: *mut ::libc::FILE = libcc2rs::stdin_unsafe();
    assert!(!((in_).is_null()));
    (unsafe {
        libc::printf(
            c"%s\n".as_ptr() as *const libc::c_char,
            (c"printf".as_ptr()),
        )
    });
    (unsafe { libc::printf(c"hello world".as_ptr() as *const libc::c_char) });
    let mut s: Vec<libc::c_char> = {
        let s = c"a string".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    (unsafe { libc::printf(c"%s\n".as_ptr() as *const libc::c_char, (s.as_mut_ptr())) });
    (unsafe {
        libc::printf(
            c"%s\n".as_ptr() as *const libc::c_char,
            ((unsafe {
                fn_0({
                    let s = c"foo".as_ptr();
                    std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                        .to_vec()
                })
            })
            .as_ptr()),
        )
    });
    (unsafe {
        libc::printf(
            c"%s\n".as_ptr() as *const libc::c_char,
            ((*(unsafe { fn2_1(&s) })).as_ptr()),
        )
    });
    (unsafe {
        log_to_2(
            libcc2rs::stdout_unsafe(),
            c"%s %d\n".as_ptr(),
            c"runtime".as_ptr(),
            4,
        )
    });
    (unsafe { log_to_3(libcc2rs::stderr_unsafe(), c"%d\n".as_ptr(), 5) });
    (unsafe { log_fmt_4(c"%d\n".as_ptr(), 6) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
