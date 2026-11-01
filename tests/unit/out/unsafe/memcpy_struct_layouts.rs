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
pub struct packed {
    pub a: i32,
    pub b: libc::c_char,
    pub c: libc::c_char,
    pub d: i16,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct reordered {
    pub a: libc::c_char,
    pub b: i32,
    pub c: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tail {
    pub a: libc::c_char,
    pub b: f64,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct nested {
    pub t: tail,
    pub c: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct array {
    pub name: [libc::c_char; 3],
    pub x: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [u8; 64] = [0_u8; 64];
    let mut p: [packed; 2] = [
        packed {
            a: 1,
            b: (2 as libc::c_char),
            c: (3 as libc::c_char),
            d: 4_i16,
        },
        packed {
            a: 5,
            b: (6 as libc::c_char),
            c: (7 as libc::c_char),
            d: 8_i16,
        },
    ];
    match ({ (buf.as_mut_ptr() as *mut ::libc::c_void) }, {
        (p.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_0, mut __tmp_1) => {
            if ::std::mem::size_of::<[packed; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_1,
                    __tmp_0,
                    ::std::mem::size_of::<[packed; 2]>() as usize,
                )
            }
            __tmp_0
        }
    };
    assert!(
        ((((buf[((::std::mem::size_of::<packed>() as usize).wrapping_add(4_usize))] as i32) == (6))
            as i32)
            != 0)
    );
    let mut p2: [packed; 2] = [<packed>::default(); 2];
    match ({ (p2.as_mut_ptr() as *mut ::libc::c_void) }, {
        (buf.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_2, mut __tmp_3) => {
            if ::std::mem::size_of::<[packed; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_3,
                    __tmp_2,
                    ::std::mem::size_of::<[packed; 2]>() as usize,
                )
            }
            __tmp_2
        }
    };
    assert!(
        (((((((((((((p2[(1) as usize].a) == (5)) as i32) != 0)
            && ((((p2[(1) as usize].b as i32) == (6)) as i32) != 0)) as i32)
            != 0)
            && ((((p2[(1) as usize].c as i32) == (7)) as i32) != 0)) as i32)
            != 0)
            && ((((p2[(1) as usize].d as i32) == (8)) as i32) != 0)) as i32)
            != 0)
    );
    let mut r: [reordered; 2] = [
        reordered {
            a: (1 as libc::c_char),
            b: 2,
            c: (3 as libc::c_char),
        },
        reordered {
            a: (4 as libc::c_char),
            b: 5,
            c: (6 as libc::c_char),
        },
    ];
    match ({ (buf.as_mut_ptr() as *mut ::libc::c_void) }, {
        (r.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_4, mut __tmp_5) => {
            if ::std::mem::size_of::<[reordered; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_5,
                    __tmp_4,
                    ::std::mem::size_of::<[reordered; 2]>() as usize,
                )
            }
            __tmp_4
        }
    };
    assert!(
        ((((buf[((::std::mem::size_of::<reordered>() as usize).wrapping_add(8_usize))] as i32)
            == (6)) as i32)
            != 0)
    );
    let mut r2: [reordered; 2] = [<reordered>::default(); 2];
    match ({ (r2.as_mut_ptr() as *mut ::libc::c_void) }, {
        (buf.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_6, mut __tmp_7) => {
            if ::std::mem::size_of::<[reordered; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_7,
                    __tmp_6,
                    ::std::mem::size_of::<[reordered; 2]>() as usize,
                )
            }
            __tmp_6
        }
    };
    assert!(
        ((((((((((r2[(1) as usize].a as i32) == (4)) as i32) != 0)
            && ((((r2[(1) as usize].b) == (5)) as i32) != 0)) as i32)
            != 0)
            && ((((r2[(1) as usize].c as i32) == (6)) as i32) != 0)) as i32)
            != 0)
    );
    let mut n: [nested; 2] = [
        nested {
            t: tail {
                a: (1 as libc::c_char),
                b: 2.5_f64,
            },
            c: (3 as libc::c_char),
        },
        nested {
            t: tail {
                a: (4 as libc::c_char),
                b: 5.5_f64,
            },
            c: (6 as libc::c_char),
        },
    ];
    match ({ (buf.as_mut_ptr() as *mut ::libc::c_void) }, {
        (n.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_8, mut __tmp_9) => {
            if ::std::mem::size_of::<[nested; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_9,
                    __tmp_8,
                    ::std::mem::size_of::<[nested; 2]>() as usize,
                )
            }
            __tmp_8
        }
    };
    assert!(
        ((((buf[((::std::mem::size_of::<nested>() as usize).wrapping_add(16_usize))] as i32) == (6))
            as i32)
            != 0)
    );
    let mut n2: [nested; 2] = [<nested>::default(); 2];
    match ({ (n2.as_mut_ptr() as *mut ::libc::c_void) }, {
        (buf.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_10, mut __tmp_11) => {
            if ::std::mem::size_of::<[nested; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_11,
                    __tmp_10,
                    ::std::mem::size_of::<[nested; 2]>() as usize,
                )
            }
            __tmp_10
        }
    };
    assert!(
        ((((((((((n2[(1) as usize].t.a as i32) == (4)) as i32) != 0)
            && ((((n2[(1) as usize].t.b) == (5.5_f64)) as i32) != 0)) as i32)
            != 0)
            && ((((n2[(1) as usize].c as i32) == (6)) as i32) != 0)) as i32)
            != 0)
    );
    let mut a: [array; 2] = [
        array {
            name: std::mem::transmute(*b"ab\0"),
            x: 1,
        },
        array {
            name: std::mem::transmute(*b"cd\0"),
            x: 2,
        },
    ];
    match ({ (buf.as_mut_ptr() as *mut ::libc::c_void) }, {
        (a.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_12, mut __tmp_13) => {
            if ::std::mem::size_of::<[array; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_13,
                    __tmp_12,
                    ::std::mem::size_of::<[array; 2]>() as usize,
                )
            }
            __tmp_12
        }
    };
    assert!(
        ((((buf[((::std::mem::size_of::<array>() as usize).wrapping_add(1_usize))] as i32)
            == ('d' as i32)) as i32)
            != 0)
    );
    let mut a2: [array; 2] = [<array>::default(); 2];
    match ({ (a2.as_mut_ptr() as *mut ::libc::c_void) }, {
        (buf.as_mut_ptr() as *const ::libc::c_void)
    }) {
        (mut __tmp_14, mut __tmp_15) => {
            if ::std::mem::size_of::<[array; 2]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_15,
                    __tmp_14,
                    ::std::mem::size_of::<[array; 2]>() as usize,
                )
            }
            __tmp_14
        }
    };
    assert!(
        ((((((((((a2[(1) as usize].name[(1) as usize] as i32) == ('d' as i32)) as i32) != 0)
            && ((((a2[(1) as usize].name[(2) as usize] as i32) == (0)) as i32) != 0))
            as i32)
            != 0)
            && ((((a2[(1) as usize].x) == (2)) as i32) != 0)) as i32)
            != 0)
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
