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
    return std::slice::from_raw_parts(
        c"hello".as_ptr(),
        (0..)
            .take_while(|&i| *c"hello".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"hello".as_ptr(),
        (0..)
            .take_while(|&i| *c"hello".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let mut calls: i32 = 0;
    let mut out: Vec<libc::c_char> = match (
        ((unsafe { receiver_0(&mut s, &mut calls) }) as *const Vec<libc::c_char>),
        { (calls as usize) },
    ) {
        (__tmp_0, mut __tmp_1) => {
            let __range = __tmp_1
                ..::std::cmp::min(
                    __tmp_1 + 2_usize,
                    (*(__tmp_0 as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(__tmp_0 as *const Vec<libc::c_char>))[__range].to_vec();
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
    out = match ((&mut (unsafe { temporary_1(&mut calls) }) as *const Vec<libc::c_char>),) {
        (__tmp_2,) => {
            let __range = 1_usize
                ..::std::cmp::min(
                    1_usize + 2_usize,
                    (*(__tmp_2 as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(__tmp_2 as *const Vec<libc::c_char>))[__range].to_vec();
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
    out = match ({
        ({
            s[(0_usize)] = ('j' as libc::c_char);
            0
        } as usize)
    },)
    {
        (mut __tmp_3,) => {
            let __range = __tmp_3
                ..::std::cmp::min(
                    __tmp_3 + 2_usize,
                    (*(&mut s as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(&mut s as *const Vec<libc::c_char>))[__range].to_vec();
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
    out = {
        let __range = 0_usize
            ..::std::cmp::min(
                0_usize
                    + ({
                        s[(1_usize)] = ('a' as libc::c_char);
                        2
                    } as usize),
                (*(&mut s as *const Vec<libc::c_char>)).len() - 1,
            );
        let mut __tmp1 = (&*(&mut s as *const Vec<libc::c_char>))[__range].to_vec();
        __tmp1.push(0);
        __tmp1
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
    out = {
        let __range = 1_usize
            ..::std::cmp::min(
                1_usize + 2_usize,
                (*(p as *const Vec<libc::c_char>)).len() - 1,
            );
        let mut __tmp1 = (&*(p as *const Vec<libc::c_char>))[__range].to_vec();
        __tmp1.push(0);
        __tmp1
    };
    (unsafe {
        libc::printf(
            c"pointer: %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
        )
    });
    out = match ((&mut match ((&mut std::slice::from_raw_parts(
        c"hello".as_ptr(),
        (0..)
            .take_while(|&i| *c"hello".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec() as *const Vec<libc::c_char>),)
    {
        (__tmp_5,) => {
            let __range = 1_usize
                ..::std::cmp::min(
                    1_usize + 3_usize,
                    (*(__tmp_5 as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(__tmp_5 as *const Vec<libc::c_char>))[__range].to_vec();
            __tmp1.push(0);
            __tmp1
        }
    } as *const Vec<libc::c_char>),)
    {
        (__tmp_4,) => {
            let __range = 1_usize
                ..::std::cmp::min(
                    1_usize + 1_usize,
                    (*(__tmp_4 as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(__tmp_4 as *const Vec<libc::c_char>))[__range].to_vec();
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
    out = match ({ (s.len() - 1) },) {
        (mut __tmp_6,) => {
            let __range = __tmp_6
                ..::std::cmp::min(
                    __tmp_6 + 0_usize,
                    (*(&mut s as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(&mut s as *const Vec<libc::c_char>))[__range].to_vec();
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
    let mut field: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"hello".as_ptr(),
        (0..)
            .take_while(|&i| *c"hello".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let mut text: Text = Text { value: &mut field };
    out = {
        let __range = 1_usize
            ..::std::cmp::min(
                1_usize + 2_usize,
                (*(text.value as *const Vec<libc::c_char>)).len() - 1,
            );
        let mut __tmp1 = (&*(text.value as *const Vec<libc::c_char>))[__range].to_vec();
        __tmp1.push(0);
        __tmp1
    };
    (unsafe {
        libc::printf(
            c"field: %s %s\n".as_ptr() as *const libc::c_char,
            (out.as_ptr()),
            ((*text.value).as_ptr()),
        )
    });
    let mut strings: Vec<Vec<libc::c_char>> = Vec::from([
        std::slice::from_raw_parts(
            c"hello".as_ptr(),
            (0..)
                .take_while(|&i| *c"hello".as_ptr().add(i) != 0)
                .count()
                + 1,
        )
        .to_vec(),
        std::slice::from_raw_parts(
            c"world".as_ptr(),
            (0..)
                .take_while(|&i| *c"world".as_ptr().add(i) != 0)
                .count()
                + 1,
        )
        .to_vec(),
    ]);
    let mut index: usize = 0_usize;
    out = match ((&mut strings[(index.postfix_inc())] as *const Vec<libc::c_char>),) {
        (__tmp_7,) => {
            let __range = 1_usize
                ..::std::cmp::min(
                    1_usize + 2_usize,
                    (*(__tmp_7 as *const Vec<libc::c_char>)).len() - 1,
                );
            let mut __tmp1 = (&*(__tmp_7 as *const Vec<libc::c_char>))[__range].to_vec();
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
