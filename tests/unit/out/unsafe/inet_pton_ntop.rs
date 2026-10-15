extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_s6_addr_access_0() {
    let loopback: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        1_u8,
    ];
    let mut addr: ::libc::in6_addr = unsafe { std::mem::zeroed() };
    let mut ap: *mut ::libc::in6_addr = (&mut addr as *mut ::libc::in6_addr);
    {
        if 16_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (loopback.as_ptr() as *const ::libc::c_void),
                ((*&raw mut (*&mut addr).s6_addr).as_mut_ptr() as *mut ::libc::c_void),
                16_usize as usize,
            )
        }
        ((*&raw mut (*&mut addr).s6_addr).as_mut_ptr() as *mut ::libc::c_void)
    };
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((*&raw mut (*ap).s6_addr).as_mut_ptr() as *const ::libc::c_void) as *const u8,
                16_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (loopback.as_ptr() as *const ::libc::c_void) as *const u8,
                16_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    (*&raw mut (*&mut addr).s6_addr)[(0) as usize] = 32_u8;
    (*&raw mut (*ap).s6_addr)[(1) as usize] = 1_u8;
    assert!(
        ((((((((*&raw mut (*&mut addr).s6_addr)[(0) as usize] as i32) == (32)) as i32) != 0)
            && (((((*&raw mut (*ap).s6_addr)[(1) as usize] as i32) == (1)) as i32) != 0))
            as i32)
            != 0)
    );
    (*&raw mut (*&mut addr).s6_addr)[(15) as usize] =
        (((*&raw mut (*&mut addr).s6_addr)[(15) as usize] as i32) + 4) as u8;
    (*&raw mut (*ap).s6_addr)[(15) as usize].postfix_inc();
    assert!((((((*&raw mut (*&mut addr).s6_addr)[(15) as usize] as i32) == (6)) as i32) != 0));
    let mut bytes: *mut u8 = (*&raw mut (*&mut addr).s6_addr).as_mut_ptr();
    (*bytes.offset((2) as isize)) = 7_u8;
    assert!((((((*&raw mut (*ap).s6_addr)[(2) as usize] as i32) == (7)) as i32) != 0));
    let mut fourth: *mut u8 = (&mut (*&raw mut (*ap).s6_addr)[(3) as usize] as *mut u8);
    (*fourth) = 8_u8;
    assert!((((((*&raw mut (*&mut addr).s6_addr)[(3) as usize] as i32) == (8)) as i32) != 0));
    {
        if 16_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (loopback.as_ptr() as *const ::libc::c_void),
                ((*&raw mut (*ap).s6_addr).as_mut_ptr() as *mut ::libc::c_void),
                16_usize as usize,
            )
        }
        ((*&raw mut (*ap).s6_addr).as_mut_ptr() as *mut ::libc::c_void)
    };
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((*&raw mut (*&mut addr).s6_addr).as_mut_ptr() as *const ::libc::c_void)
                    as *const u8,
                16_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (loopback.as_ptr() as *const ::libc::c_void) as *const u8,
                16_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_s6_addr_access_0() });
    let mut buf: [u8; 16] = [0_u8; 16];
    assert!(
        (((({
            unsafe extern "C" {
                fn inet_pton(af: i32, src: *const libc::c_char, dst: *mut ::libc::c_void) -> i32;
            }
            inet_pton(
                libc::AF_INET,
                (c"1.2.3.4".as_ptr().cast_mut()).cast_const(),
                (buf.as_mut_ptr() as *mut ::libc::c_void),
            )
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        (((((((((((((buf[(0) as usize] as i32) == (1)) as i32) != 0)
            && ((((buf[(1) as usize] as i32) == (2)) as i32) != 0)) as i32)
            != 0)
            && ((((buf[(2) as usize] as i32) == (3)) as i32) != 0)) as i32)
            != 0)
            && ((((buf[(3) as usize] as i32) == (4)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((({
            unsafe extern "C" {
                fn inet_pton(af: i32, src: *const libc::c_char, dst: *mut ::libc::c_void) -> i32;
            }
            inet_pton(
                libc::AF_INET,
                (c"999.1.1.1".as_ptr().cast_mut()).cast_const(),
                (buf.as_mut_ptr() as *mut ::libc::c_void),
            )
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        (((({
            unsafe extern "C" {
                fn inet_pton(af: i32, src: *const libc::c_char, dst: *mut ::libc::c_void) -> i32;
            }
            inet_pton(
                libc::AF_INET,
                (c"not an ip".as_ptr().cast_mut()).cast_const(),
                (buf.as_mut_ptr() as *mut ::libc::c_void),
            )
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        (((({
            unsafe extern "C" {
                fn inet_pton(af: i32, src: *const libc::c_char, dst: *mut ::libc::c_void) -> i32;
            }
            inet_pton(
                libc::AF_INET6,
                (c"::1".as_ptr().cast_mut()).cast_const(),
                (buf.as_mut_ptr() as *mut ::libc::c_void),
            )
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        (((((((buf[(0) as usize] as i32) == (0)) as i32) != 0)
            && ((((buf[(15) as usize] as i32) == (1)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((({
            unsafe extern "C" {
                fn inet_pton(af: i32, src: *const libc::c_char, dst: *mut ::libc::c_void) -> i32;
            }
            inet_pton(
                libc::AF_INET6,
                (c"2001:db8::5".as_ptr().cast_mut()).cast_const(),
                (buf.as_mut_ptr() as *mut ::libc::c_void),
            )
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((((((((buf[(0) as usize] as i32) == (32)) as i32) != 0)
            && ((((buf[(1) as usize] as i32) == (1)) as i32) != 0)) as i32)
            != 0)
            && ((((buf[(15) as usize] as i32) == (5)) as i32) != 0)) as i32)
            != 0)
    );
    let mut text: [libc::c_char; 64] = [(0 as libc::c_char); 64];
    let mut four: [u8; 4] = [10_u8, 0_u8, 0_u8, 1_u8];
    assert!(
        ((((libc::strcmp(
            {
                unsafe extern "C" {
                    fn inet_ntop(
                        af: i32,
                        src: *const ::libc::c_void,
                        dst: *mut libc::c_char,
                        size: u32,
                    ) -> *const libc::c_char;
                }
                inet_ntop(
                    libc::AF_INET,
                    (four.as_mut_ptr() as *const ::libc::c_void),
                    text.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 64]>() as u32),
                )
            },
            (c"10.0.0.1".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    let mut sixteen: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ];
    sixteen[(15) as usize] = 1_u8;
    assert!(
        ((((libc::strcmp(
            {
                unsafe extern "C" {
                    fn inet_ntop(
                        af: i32,
                        src: *const ::libc::c_void,
                        dst: *mut libc::c_char,
                        size: u32,
                    ) -> *const libc::c_char;
                }
                inet_ntop(
                    libc::AF_INET6,
                    (sixteen.as_mut_ptr() as *const ::libc::c_void),
                    text.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 64]>() as u32),
                )
            },
            (c"::1".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        (((({
            unsafe extern "C" {
                fn inet_ntop(
                    af: i32,
                    src: *const ::libc::c_void,
                    dst: *mut libc::c_char,
                    size: u32,
                ) -> *const libc::c_char;
            }
            inet_ntop(
                libc::AF_INET,
                (four.as_mut_ptr() as *const ::libc::c_void),
                text.as_mut_ptr(),
                4_u32,
            )
        })
        .is_null()) as i32)
            != 0)
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
