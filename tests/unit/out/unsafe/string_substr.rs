extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Text {
    pub value: *mut Vec<libc::c_char>,
}
pub unsafe fn receiver_0(s: *mut Vec<libc::c_char>, calls: *mut i32) -> *mut Vec<libc::c_char> {
    (*calls).prefix_inc();
    return s;
}
pub unsafe fn temporary_1(calls: *mut i32) -> Vec<libc::c_char> {
    (*calls).prefix_inc();
    return {
        let s = c"hello".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Vec<libc::c_char> = {
        let s = c"hello".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut calls: i32 = 0;
    let mut out: Vec<libc::c_char> = match (
        ((unsafe { receiver_0(&mut s, &mut calls) }) as *const Vec<libc::c_char>),
        (calls as usize),
        2_usize,
    ) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"receiver: %s %d\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            (calls),
        )
    });
    calls = 0;
    out = match (
        (&mut (unsafe { temporary_1(&mut calls) }) as *const Vec<libc::c_char>),
        1_usize,
        2_usize,
    ) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"temporary: %s %d\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            (calls),
        )
    });
    out = match (
        (&mut s as *const Vec<libc::c_char>),
        ({
            s[(0_usize)] = ('j' as libc::c_char);
            0
        } as usize),
        2_usize,
    ) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"position mutation: %s %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            (s.as_ptr()),
        )
    });
    out = match (
        (&mut s as *const Vec<libc::c_char>),
        0_usize,
        ({
            s[(1_usize)] = ('a' as libc::c_char);
            2
        } as usize),
    ) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"count mutation: %s %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            (s.as_ptr()),
        )
    });
    out[(0_usize)] = ('X' as libc::c_char);
    s[(1_usize)] = ('b' as libc::c_char);
    (unsafe {
        libc::printf(
            c"independent: %s %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            (s.as_ptr()),
        )
    });
    let mut p: *const Vec<libc::c_char> = (&mut s as *mut Vec<libc::c_char>).cast_const();
    out = match ((p as *const Vec<libc::c_char>), 1_usize, 2_usize) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"pointer: %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
        )
    });
    out = match (
        (&mut match (
            (&mut {
                let s = c"hello".as_ptr();
                std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                    .to_vec()
            } as *const Vec<libc::c_char>),
            1_usize,
            3_usize,
        ) {
            (__source, __pos, __count) => {
                let __s = &*__source;
                let mut __tmp1 =
                    __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
                __tmp1.push(0);
                __tmp1
            }
        } as *const Vec<libc::c_char>),
        1_usize,
        1_usize,
    ) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"nested: %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
        )
    });
    out = match ((&mut s as *const Vec<libc::c_char>), (s.len() - 1), 0_usize) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"empty: %zu\n".as_ptr() as *const libc::c_char,
            (out.len() - 1),
        )
    });
    let mut field: Vec<libc::c_char> = {
        let s = c"hello".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut text: Text = Text { value: &mut field };
    out = match ((text.value as *const Vec<libc::c_char>), 1_usize, 2_usize) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"field: %s %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            ((*text.value).as_ptr()),
        )
    });
    let mut strings: Vec<Vec<libc::c_char>> = Vec::from([
        {
            let s = c"hello".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        },
        {
            let s = c"world".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        },
    ]);
    let mut index: usize = 0_usize;
    out = match (
        (&mut strings[(index.postfix_inc())] as *const Vec<libc::c_char>),
        1_usize,
        2_usize,
    ) {
        (__source, __pos, __count) => {
            let __s = &*__source;
            let mut __tmp1 = __s[__pos..::std::cmp::min(__pos + __count, __s.len() - 1)].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    };
    (unsafe {
        libc::printf(
            c"element: %s %zu\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            (index),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
