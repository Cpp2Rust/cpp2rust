extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_memcpy_0() {
    let src: [u8; 6] = *b"hello\0";
    let mut dst: [u8; 6] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    let mut r: *mut ::libc::c_void = {
        if 6_u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                (src.as_ptr() as *const u8 as *const ::libc::c_void),
                (dst.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
                6_u64 as usize,
            )
        }
        (dst.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
    };
    assert!(((r) == (dst.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)));
    assert!(
        (((dst[(0) as usize] as i32) == (('h' as u8) as i32))
            && ((dst[(1) as usize] as i32) == (('e' as u8) as i32)))
            && ((dst[(2) as usize] as i32) == (('l' as u8) as i32))
    );
    assert!(
        (((dst[(3) as usize] as i32) == (('l' as u8) as i32))
            && ((dst[(4) as usize] as i32) == (('o' as u8) as i32)))
            && ((dst[(5) as usize] as i32) == (('\0' as u8) as i32))
    );
}
pub unsafe fn test_memset_1() {
    let mut buf: [u8; 4] = [0_u8; 4];
    let mut r: *mut ::libc::c_void = {
        let byte_0 = (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..4_u64 {
            *byte_0.offset(offset as isize) = (('x' as u8) as i32) as u8;
        }
        (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
    };
    assert!(((r) == (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)));
    assert!(
        ((((buf[(0) as usize] as i32) == (('x' as u8) as i32))
            && ((buf[(1) as usize] as i32) == (('x' as u8) as i32)))
            && ((buf[(2) as usize] as i32) == (('x' as u8) as i32)))
            && ((buf[(3) as usize] as i32) == (('x' as u8) as i32))
    );
}
pub unsafe fn test_memcmp_2() {
    let a: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let b: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let c: [u8; 4] = [1_u8, 2_u8, 9_u8, 4_u8];
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (a.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (b.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_u64 as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (a.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_u64 as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) < (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (c.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (a.as_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_u64 as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) > (0))
    );
}
pub unsafe fn test_memmove_3() {
    let mut buf: [u8; 6] = [
        ('a' as u8),
        ('b' as u8),
        ('c' as u8),
        ('d' as u8),
        ('e' as u8),
        ('\0' as u8),
    ];
    let mut r: *mut ::libc::c_void = {
        if 4_u64 != 0 {
            ::std::ptr::copy_nonoverlapping(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void),
                (buf.as_mut_ptr().offset((1) as isize) as *mut u8 as *mut ::libc::c_void),
                4_u64 as usize,
            )
        }
        (buf.as_mut_ptr().offset((1) as isize) as *mut u8 as *mut ::libc::c_void)
    };
    assert!(((r) == (buf.as_mut_ptr().offset((1) as isize) as *mut u8 as *mut ::libc::c_void)));
    assert!(
        (((buf[(0) as usize] as i32) == (('a' as u8) as i32))
            && ((buf[(1) as usize] as i32) == (('a' as u8) as i32)))
            && ((buf[(2) as usize] as i32) == (('b' as u8) as i32))
    );
    assert!(
        (((buf[(3) as usize] as i32) == (('c' as u8) as i32))
            && ((buf[(4) as usize] as i32) == (('d' as u8) as i32)))
            && ((buf[(5) as usize] as i32) == (('\0' as u8) as i32))
    );
}
pub unsafe fn test_strchr_4() {
    let mut s: *const u8 = b"hello world\0".as_ptr();
    let mut r: *const u8 = libc::strchr(s as *const i8, (('w' as u8) as i32)) as *const u8;
    assert!(!((r).is_null()));
    assert!((((*r) as i32) == (('w' as u8) as i32)));
    assert!((libc::strchr(s as *const i8, (('z' as u8) as i32)) as *const u8).is_null());
}
pub unsafe fn test_strlen_5() {
    assert!(((libc::strlen(b"\0".as_ptr() as *const i8) as u64) == (0_u64)));
    assert!(((libc::strlen(b"hello\0".as_ptr() as *const i8) as u64) == (5_u64)));
    assert!(((libc::strlen(b"hello world\0".as_ptr() as *const i8) as u64) == (11_u64)));
}
pub unsafe fn test_strcmp_6() {
    assert!(
        ((libc::strcmp(
            b"abc\0".as_ptr() as *const i8,
            b"abc\0".as_ptr() as *const i8
        )) == (0))
    );
    assert!(
        ((libc::strcmp(
            b"abc\0".as_ptr() as *const i8,
            b"abd\0".as_ptr() as *const i8
        )) < (0))
    );
    assert!(
        ((libc::strcmp(
            b"abd\0".as_ptr() as *const i8,
            b"abc\0".as_ptr() as *const i8
        )) > (0))
    );
    let mut p: *const u8 = b"abc\0".as_ptr();
    let mut q: *const u8 = b"abd\0".as_ptr();
    let mut buf: [u8; 4] = [('a' as u8), ('b' as u8), ('c' as u8), ('\0' as u8)];
    assert!(((libc::strcmp(p as *const i8, p as *const i8)) == (0)));
    assert!(((libc::strcmp(p as *const i8, q as *const i8)) < (0)));
    assert!(((libc::strcmp((buf.as_mut_ptr()).cast_const() as *const i8, p as *const i8)) == (0)));
}
pub unsafe fn test_strncmp_7() {
    assert!(
        ((libc::strncmp(
            b"abcdef\0".as_ptr() as *const i8,
            b"abcxyz\0".as_ptr() as *const i8,
            3_u64 as usize
        )) == (0))
    );
    assert!(
        ((libc::strncmp(
            b"abcdef\0".as_ptr() as *const i8,
            b"abcxyz\0".as_ptr() as *const i8,
            4_u64 as usize
        )) < (0))
    );
    assert!(
        ((libc::strncmp(
            b"abcxyz\0".as_ptr() as *const i8,
            b"abcdef\0".as_ptr() as *const i8,
            4_u64 as usize
        )) > (0))
    );
    let mut p: *const u8 = b"abcdef\0".as_ptr();
    let mut q: *const u8 = b"abcxyz\0".as_ptr();
    let mut buf: [u8; 7] = [
        ('a' as u8),
        ('b' as u8),
        ('c' as u8),
        ('d' as u8),
        ('e' as u8),
        ('f' as u8),
        ('\0' as u8),
    ];
    let mut n: u64 = 3_u64;
    assert!(((libc::strncmp(p as *const i8, q as *const i8, n as usize)) == (0)));
    assert!(
        ((libc::strncmp(
            p as *const i8,
            q as *const i8,
            (n).wrapping_add(1_u64) as usize
        )) < (0))
    );
    assert!(
        ((libc::strncmp(
            (buf.as_mut_ptr()).cast_const() as *const i8,
            p as *const i8,
            6_u64 as usize
        )) == (0))
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_memcpy_0() });
    (unsafe { test_memset_1() });
    (unsafe { test_memcmp_2() });
    (unsafe { test_memmove_3() });
    (unsafe { test_strchr_4() });
    (unsafe { test_strlen_5() });
    (unsafe { test_strcmp_6() });
    (unsafe { test_strncmp_7() });
    return 0;
}
