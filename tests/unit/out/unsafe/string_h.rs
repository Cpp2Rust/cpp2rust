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
    assert!(((((r) == (dst.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)) as i32) != 0));
    assert!(
        ((((((((((dst[(0) as usize] as i32) == ('h' as i32)) as i32) != 0)
            && ((((dst[(1) as usize] as i32) == ('e' as i32)) as i32) != 0)) as i32)
            != 0)
            && ((((dst[(2) as usize] as i32) == ('l' as i32)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((((dst[(3) as usize] as i32) == ('l' as i32)) as i32) != 0)
            && ((((dst[(4) as usize] as i32) == ('o' as i32)) as i32) != 0)) as i32)
            != 0)
            && ((((dst[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0)) as i32)
            != 0)
    );
}
pub unsafe fn test_memset_1() {
    let mut buf: [u8; 4] = [0_u8; 4];
    let mut r: *mut ::libc::c_void = {
        let byte_0 = (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..4_u64 {
            *byte_0.offset(offset as isize) = ('x' as i32) as u8;
        }
        (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
    };
    assert!(((((r) == (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)) as i32) != 0));
    assert!(
        (((((((((((((buf[(0) as usize] as i32) == ('x' as i32)) as i32) != 0)
            && ((((buf[(1) as usize] as i32) == ('x' as i32)) as i32) != 0))
            as i32)
            != 0)
            && ((((buf[(2) as usize] as i32) == ('x' as i32)) as i32) != 0)) as i32)
            != 0)
            && ((((buf[(3) as usize] as i32) == ('x' as i32)) as i32) != 0)) as i32)
            != 0)
    );
}
pub unsafe fn test_memcmp_2() {
    let a: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let b: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let c: [u8; 4] = [1_u8, 2_u8, 9_u8, 4_u8];
    assert!(
        (((({
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
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        (((({
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
        }) < (0)) as i32)
            != 0)
    );
    assert!(
        (((({
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
        }) > (0)) as i32)
            != 0)
    );
}
pub unsafe fn test_memmove_3() {
    let mut buf: [u8; 6] = [
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('d' as i32) as u8),
        (('e' as i32) as u8),
        (('\0' as i32) as u8),
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
    assert!(
        ((((r) == (buf.as_mut_ptr().offset((1) as isize) as *mut u8 as *mut ::libc::c_void))
            as i32)
            != 0)
    );
    assert!(
        ((((((((((buf[(0) as usize] as i32) == ('a' as i32)) as i32) != 0)
            && ((((buf[(1) as usize] as i32) == ('a' as i32)) as i32) != 0)) as i32)
            != 0)
            && ((((buf[(2) as usize] as i32) == ('b' as i32)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((((buf[(3) as usize] as i32) == ('c' as i32)) as i32) != 0)
            && ((((buf[(4) as usize] as i32) == ('d' as i32)) as i32) != 0)) as i32)
            != 0)
            && ((((buf[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0)) as i32)
            != 0)
    );
}
pub unsafe fn test_strchr_4() {
    let mut s: *const u8 = (b"hello world\0".as_ptr().cast_mut()).cast_const();
    let mut r: *mut u8 =
        libc::strchr((s as *mut u8).cast_const() as *const i8, ('w' as i32)) as *mut u8;
    assert!((((!((r).is_null())) as i32) != 0));
    assert!((((((*r) as i32) == ('w' as i32)) as i32) != 0));
    assert!(
        ((((libc::strchr((s as *mut u8).cast_const() as *const i8, ('z' as i32)) as *mut u8)
            .is_null()) as i32)
            != 0)
    );
}
pub unsafe fn test_strlen_5() {
    assert!(
        ((((libc::strlen((b"\0".as_ptr().cast_mut()).cast_const() as *const i8) as u64) == (0_u64))
            as i32)
            != 0)
    );
    assert!(
        ((((libc::strlen((b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8) as u64)
            == (5_u64)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strlen((b"hello world\0".as_ptr().cast_mut()).cast_const() as *const i8) as u64)
            == (11_u64)) as i32)
            != 0)
    );
}
pub unsafe fn test_strcmp_6() {
    assert!(
        ((((libc::strcmp(
            (b"abc\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"abc\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (b"abc\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"abd\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) < (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (b"abd\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"abc\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) > (0)) as i32)
            != 0)
    );
    let mut p: *const u8 = (b"abc\0".as_ptr().cast_mut()).cast_const();
    let mut q: *const u8 = (b"abd\0".as_ptr().cast_mut()).cast_const();
    let mut buf: [u8; 4] = [
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('\0' as i32) as u8),
    ];
    assert!(((((libc::strcmp(p as *const i8, p as *const i8)) == (0)) as i32) != 0));
    assert!(((((libc::strcmp(p as *const i8, q as *const i8)) < (0)) as i32) != 0));
    assert!(
        ((((libc::strcmp((buf.as_mut_ptr()).cast_const() as *const i8, p as *const i8)) == (0))
            as i32)
            != 0)
    );
}
pub unsafe fn test_strncmp_7() {
    assert!(
        ((((libc::strncmp(
            (b"abcdef\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"abcxyz\0".as_ptr().cast_mut()).cast_const() as *const i8,
            3_u64 as usize
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strncmp(
            (b"abcdef\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"abcxyz\0".as_ptr().cast_mut()).cast_const() as *const i8,
            4_u64 as usize
        )) < (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strncmp(
            (b"abcxyz\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"abcdef\0".as_ptr().cast_mut()).cast_const() as *const i8,
            4_u64 as usize
        )) > (0)) as i32)
            != 0)
    );
    let mut p: *const u8 = (b"abcdef\0".as_ptr().cast_mut()).cast_const();
    let mut q: *const u8 = (b"abcxyz\0".as_ptr().cast_mut()).cast_const();
    let mut buf: [u8; 7] = [
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('d' as i32) as u8),
        (('e' as i32) as u8),
        (('f' as i32) as u8),
        (('\0' as i32) as u8),
    ];
    let mut n: u64 = 3_u64;
    assert!(((((libc::strncmp(p as *const i8, q as *const i8, n as usize)) == (0)) as i32) != 0));
    assert!(
        ((((libc::strncmp(
            p as *const i8,
            q as *const i8,
            (n).wrapping_add(1_u64) as usize
        )) < (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strncmp(
            (buf.as_mut_ptr()).cast_const() as *const i8,
            p as *const i8,
            6_u64 as usize
        )) == (0)) as i32)
            != 0)
    );
}
pub unsafe fn test_memchr_8() {
    let data: [u8; 4] = [16_u8, 32_u8, 48_u8, 64_u8];
    let mut r: *mut ::libc::c_void = libc::memchr(
        (data.as_ptr() as *const u8 as *const ::libc::c_void) as *const ::libc::c_void,
        48,
        4_u64 as usize,
    );
    assert!(
        ((((r) == ((&data[(2) as usize] as *const u8) as *mut u8 as *mut ::libc::c_void)) as i32)
            != 0)
    );
    assert!(
        ((((libc::memchr(
            (data.as_ptr() as *const u8 as *const ::libc::c_void) as *const ::libc::c_void,
            153,
            4_u64 as usize
        ))
        .is_null()) as i32)
            != 0)
    );
    let mut p: *const ::libc::c_void = (data.as_ptr() as *const u8 as *const ::libc::c_void);
    let mut n: u64 = 4_u64;
    assert!(
        ((((libc::memchr(p as *const ::libc::c_void, 16, n as usize))
            == (p as *mut ::libc::c_void as *mut ::libc::c_void)) as i32)
            != 0)
    );
}
pub unsafe fn test_strrchr_9() {
    let mut s: *const u8 = (b"hello world\0".as_ptr().cast_mut()).cast_const();
    let mut r: *mut u8 =
        libc::strrchr((s as *mut u8).cast_const() as *const i8, ('l' as i32)) as *mut u8;
    assert!((((!((r).is_null())) as i32) != 0));
    assert!((((((*r) as i32) == ('l' as i32)) as i32) != 0));
    assert!(((((r) == (s.offset((9) as isize) as *mut u8)) as i32) != 0));
    assert!(
        ((((libc::strrchr((s as *mut u8).cast_const() as *const i8, ('z' as i32)) as *mut u8)
            .is_null()) as i32)
            != 0)
    );
    let mut buf: [u8; 4] = [
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('a' as i32) as u8),
        (('\0' as i32) as u8),
    ];
    assert!(
        ((((libc::strrchr((buf.as_mut_ptr()).cast_const() as *const i8, ('a' as i32)) as *mut u8)
            == (&mut buf[(2) as usize] as *mut u8)) as i32)
            != 0)
    );
}
pub unsafe fn test_strdup_10() {
    let mut d: *mut u8 =
        libc::strdup((b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8) as *mut u8;
    assert!((((!((d).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (d).cast_const() as *const i8,
            (b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
    libc::free((d as *mut u8 as *mut ::libc::c_void));
    let mut p: *const u8 = (b"world\0".as_ptr().cast_mut()).cast_const();
    let mut buf: [u8; 4] = [
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('\0' as i32) as u8),
    ];
    let mut d2: *mut u8 = libc::strdup(p as *const i8) as *mut u8;
    assert!((((!((d2).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp((d2).cast_const() as *const i8, p as *const i8)) == (0)) as i32) != 0)
    );
    libc::free((d2 as *mut u8 as *mut ::libc::c_void));
    let mut d3: *mut u8 = libc::strdup((buf.as_mut_ptr()).cast_const() as *const i8) as *mut u8;
    assert!((((!((d3).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (d3).cast_const() as *const i8,
            (buf.as_mut_ptr()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
    libc::free((d3 as *mut u8 as *mut ::libc::c_void));
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
    (unsafe { test_memchr_8() });
    (unsafe { test_strrchr_9() });
    (unsafe { test_strdup_10() });
    return 0;
}
