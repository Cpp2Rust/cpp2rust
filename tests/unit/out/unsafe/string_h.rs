extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_memcpy_0() {
    let src: [core::ffi::c_char; 6] = libcc2rs::char_array(b"hello\0");
    let mut dst: [core::ffi::c_char; 6] = [
        (0 as core::ffi::c_char),
        (0 as core::ffi::c_char),
        (0 as core::ffi::c_char),
        (0 as core::ffi::c_char),
        (0 as core::ffi::c_char),
        (0 as core::ffi::c_char),
    ];
    let mut r: *mut ::libc::c_void = {
        if 6_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (src.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void),
                (dst.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void),
                6_usize as usize,
            )
        }
        (dst.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((r) == (dst.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)) as i32)
            != 0)
    );
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
    let mut buf: [core::ffi::c_char; 4] = [(0 as core::ffi::c_char); 4];
    let mut r: *mut ::libc::c_void = {
        let byte_0 = (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..4_usize {
            *byte_0.offset(offset as isize) = ('x' as i32) as u8;
        }
        (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((r) == (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)) as i32)
            != 0)
    );
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
    let a: [core::ffi::c_char; 4] = [
        (1 as core::ffi::c_char),
        (2 as core::ffi::c_char),
        (3 as core::ffi::c_char),
        (4 as core::ffi::c_char),
    ];
    let b: [core::ffi::c_char; 4] = [
        (1 as core::ffi::c_char),
        (2 as core::ffi::c_char),
        (3 as core::ffi::c_char),
        (4 as core::ffi::c_char),
    ];
    let c: [core::ffi::c_char; 4] = [
        (1 as core::ffi::c_char),
        (2 as core::ffi::c_char),
        (9 as core::ffi::c_char),
        (4 as core::ffi::c_char),
    ];
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (a.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (b.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
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
                (a.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
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
                (c.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (a.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
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
    let mut buf: [core::ffi::c_char; 6] = [
        (('a' as i32) as core::ffi::c_char),
        (('b' as i32) as core::ffi::c_char),
        (('c' as i32) as core::ffi::c_char),
        (('d' as i32) as core::ffi::c_char),
        (('e' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    let mut r: *mut ::libc::c_void = {
        if 4_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (buf.as_mut_ptr() as *const core::ffi::c_char as *const ::libc::c_void),
                (buf.as_mut_ptr().offset((1) as isize) as *mut core::ffi::c_char
                    as *mut ::libc::c_void),
                4_usize as usize,
            )
        }
        (buf.as_mut_ptr().offset((1) as isize) as *mut core::ffi::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((r)
            == (buf.as_mut_ptr().offset((1) as isize) as *mut core::ffi::c_char
                as *mut ::libc::c_void)) as i32)
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
    let mut s: *const core::ffi::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut core::ffi::c_char =
        libc::strchr((s as *mut core::ffi::c_char).cast_const(), ('w' as i32));
    assert!((((!((r).is_null())) as i32) != 0));
    assert!((((((*r) as i32) == ('w' as i32)) as i32) != 0));
    assert!(
        ((((libc::strchr((s as *mut core::ffi::c_char).cast_const(), ('z' as i32))).is_null())
            as i32)
            != 0)
    );
}
pub unsafe fn test_strlen_5() {
    assert!(((((libc::strlen((c"".as_ptr().cast_mut()).cast_const())) == (0_usize)) as i32) != 0));
    assert!(
        ((((libc::strlen((c"hello".as_ptr().cast_mut()).cast_const())) == (5_usize)) as i32) != 0)
    );
    assert!(
        ((((libc::strlen((c"hello world".as_ptr().cast_mut()).cast_const())) == (11_usize))
            as i32)
            != 0)
    );
}
pub unsafe fn test_strcmp_6() {
    assert!(
        ((((libc::strcmp(
            (c"abc".as_ptr().cast_mut()).cast_const(),
            (c"abc".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (c"abc".as_ptr().cast_mut()).cast_const(),
            (c"abd".as_ptr().cast_mut()).cast_const()
        )) < (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (c"abd".as_ptr().cast_mut()).cast_const(),
            (c"abc".as_ptr().cast_mut()).cast_const()
        )) > (0)) as i32)
            != 0)
    );
    let mut p: *const core::ffi::c_char = (c"abc".as_ptr().cast_mut()).cast_const();
    let mut q: *const core::ffi::c_char = (c"abd".as_ptr().cast_mut()).cast_const();
    let mut buf: [core::ffi::c_char; 4] = [
        (('a' as i32) as core::ffi::c_char),
        (('b' as i32) as core::ffi::c_char),
        (('c' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    assert!(((((libc::strcmp(p, p)) == (0)) as i32) != 0));
    assert!(((((libc::strcmp(p, q)) < (0)) as i32) != 0));
    assert!(((((libc::strcmp((buf.as_mut_ptr()).cast_const(), p)) == (0)) as i32) != 0));
}
pub unsafe fn test_strncmp_7() {
    assert!(
        ((((libc::strncmp(
            (c"abcdef".as_ptr().cast_mut()).cast_const(),
            (c"abcxyz".as_ptr().cast_mut()).cast_const(),
            3_usize as usize
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strncmp(
            (c"abcdef".as_ptr().cast_mut()).cast_const(),
            (c"abcxyz".as_ptr().cast_mut()).cast_const(),
            4_usize as usize
        )) < (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strncmp(
            (c"abcxyz".as_ptr().cast_mut()).cast_const(),
            (c"abcdef".as_ptr().cast_mut()).cast_const(),
            4_usize as usize
        )) > (0)) as i32)
            != 0)
    );
    let mut p: *const core::ffi::c_char = (c"abcdef".as_ptr().cast_mut()).cast_const();
    let mut q: *const core::ffi::c_char = (c"abcxyz".as_ptr().cast_mut()).cast_const();
    let mut buf: [core::ffi::c_char; 7] = [
        (('a' as i32) as core::ffi::c_char),
        (('b' as i32) as core::ffi::c_char),
        (('c' as i32) as core::ffi::c_char),
        (('d' as i32) as core::ffi::c_char),
        (('e' as i32) as core::ffi::c_char),
        (('f' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    let mut n: usize = 3_usize;
    assert!(((((libc::strncmp(p, q, n as usize)) == (0)) as i32) != 0));
    assert!(((((libc::strncmp(p, q, (n).wrapping_add(1_usize) as usize)) < (0)) as i32) != 0));
    assert!(
        ((((libc::strncmp((buf.as_mut_ptr()).cast_const(), p, 6_usize as usize)) == (0)) as i32)
            != 0)
    );
}
pub unsafe fn test_memchr_8() {
    let data: [core::ffi::c_char; 4] = [
        (16 as core::ffi::c_char),
        (32 as core::ffi::c_char),
        (48 as core::ffi::c_char),
        (64 as core::ffi::c_char),
    ];
    let mut r: *mut ::libc::c_void = libc::memchr(
        (data.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void)
            as *const ::libc::c_void,
        48,
        4_usize as usize,
    );
    assert!(
        ((((r)
            == ((&data[(2) as usize] as *const core::ffi::c_char) as *mut core::ffi::c_char
                as *mut ::libc::c_void)) as i32)
            != 0)
    );
    assert!(
        ((((libc::memchr(
            (data.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void)
                as *const ::libc::c_void,
            153,
            4_usize as usize
        ))
        .is_null()) as i32)
            != 0)
    );
    let mut p: *const ::libc::c_void =
        (data.as_ptr() as *const core::ffi::c_char as *const ::libc::c_void);
    let mut n: usize = 4_usize;
    assert!(
        ((((libc::memchr(p as *const ::libc::c_void, 16, n as usize))
            == (p as *mut ::libc::c_void as *mut ::libc::c_void)) as i32)
            != 0)
    );
}
pub unsafe fn test_strrchr_9() {
    let mut s: *const core::ffi::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut core::ffi::c_char =
        libc::strrchr((s as *mut core::ffi::c_char).cast_const(), ('l' as i32));
    assert!((((!((r).is_null())) as i32) != 0));
    assert!((((((*r) as i32) == ('l' as i32)) as i32) != 0));
    assert!(((((r) == (s.offset((9) as isize) as *mut core::ffi::c_char)) as i32) != 0));
    assert!(
        ((((libc::strrchr((s as *mut core::ffi::c_char).cast_const(), ('z' as i32))).is_null())
            as i32)
            != 0)
    );
    let mut buf: [core::ffi::c_char; 4] = [
        (('a' as i32) as core::ffi::c_char),
        (('b' as i32) as core::ffi::c_char),
        (('a' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    assert!(
        ((((libc::strrchr((buf.as_mut_ptr()).cast_const(), ('a' as i32)))
            == (&mut buf[(2) as usize] as *mut core::ffi::c_char)) as i32)
            != 0)
    );
}
pub unsafe fn test_strdup_10() {
    let mut d: *mut core::ffi::c_char =
        libcc2rs::strdup_unsafe((c"hello".as_ptr().cast_mut()).cast_const());
    assert!((((!((d).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (d).cast_const(),
            (c"hello".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe((d as *mut core::ffi::c_char as *mut ::libc::c_void));
    let mut p: *const core::ffi::c_char = (c"world".as_ptr().cast_mut()).cast_const();
    let mut buf: [core::ffi::c_char; 4] = [
        (('a' as i32) as core::ffi::c_char),
        (('b' as i32) as core::ffi::c_char),
        (('c' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    let mut d2: *mut core::ffi::c_char = libcc2rs::strdup_unsafe(p);
    assert!((((!((d2).is_null())) as i32) != 0));
    assert!(((((libc::strcmp((d2).cast_const(), p)) == (0)) as i32) != 0));
    libcc2rs::free_unsafe((d2 as *mut core::ffi::c_char as *mut ::libc::c_void));
    let mut d3: *mut core::ffi::c_char = libcc2rs::strdup_unsafe((buf.as_mut_ptr()).cast_const());
    assert!((((!((d3).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp((d3).cast_const(), (buf.as_mut_ptr()).cast_const())) == (0)) as i32) != 0)
    );
    libcc2rs::free_unsafe((d3 as *mut core::ffi::c_char as *mut ::libc::c_void));
}
pub unsafe fn test_strcspn_11() {
    assert!(
        ((((libc::strcspn(
            (c"hello".as_ptr().cast_mut()).cast_const(),
            (c"el".as_ptr().cast_mut()).cast_const()
        )) == (1_usize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcspn(
            (c"abc".as_ptr().cast_mut()).cast_const(),
            (c"xyz".as_ptr().cast_mut()).cast_const()
        )) == (3_usize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcspn(
            (c"".as_ptr().cast_mut()).cast_const(),
            (c"abc".as_ptr().cast_mut()).cast_const()
        )) == (0_usize)) as i32)
            != 0)
    );
    let mut s: *const core::ffi::c_char = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut rej: *const core::ffi::c_char = (c"el".as_ptr().cast_mut()).cast_const();
    assert!(((((libc::strcspn(s, rej)) == (1_usize)) as i32) != 0));
}
pub unsafe fn test_strspn_12() {
    assert!(
        ((((libc::strspn(
            (c"hello".as_ptr().cast_mut()).cast_const(),
            (c"hel".as_ptr().cast_mut()).cast_const()
        )) == (4_usize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strspn(
            (c"abc".as_ptr().cast_mut()).cast_const(),
            (c"xyz".as_ptr().cast_mut()).cast_const()
        )) == (0_usize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strspn(
            (c"aaa".as_ptr().cast_mut()).cast_const(),
            (c"a".as_ptr().cast_mut()).cast_const()
        )) == (3_usize)) as i32)
            != 0)
    );
    let mut s: *const core::ffi::c_char = (c"hello".as_ptr().cast_mut()).cast_const();
    let mut acc: *const core::ffi::c_char = (c"hel".as_ptr().cast_mut()).cast_const();
    assert!(((((libc::strspn(s, acc)) == (4_usize)) as i32) != 0));
}
pub unsafe fn test_strstr_13() {
    let mut h: *const core::ffi::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut core::ffi::c_char = libc::strstr(
        (h as *mut core::ffi::c_char).cast_const(),
        (c"world".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((r).is_null())) as i32) != 0));
    assert!(((((r) == (h.offset((6) as isize) as *mut core::ffi::c_char)) as i32) != 0));
    assert!(
        ((((libc::strstr(
            (h as *mut core::ffi::c_char).cast_const(),
            (c"xyz".as_ptr().cast_mut()).cast_const()
        ))
        .is_null()) as i32)
            != 0)
    );
    let mut buf: [core::ffi::c_char; 6] = [
        (('h' as i32) as core::ffi::c_char),
        (('e' as i32) as core::ffi::c_char),
        (('l' as i32) as core::ffi::c_char),
        (('l' as i32) as core::ffi::c_char),
        (('o' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    assert!(
        ((((libc::strstr(
            (buf.as_mut_ptr()).cast_const(),
            (c"ll".as_ptr().cast_mut()).cast_const()
        )) == (&mut buf[(2) as usize] as *mut core::ffi::c_char)) as i32)
            != 0)
    );
}
pub unsafe fn test_strpbrk_14() {
    let mut s: *const core::ffi::c_char = (c"hello world".as_ptr().cast_mut()).cast_const();
    let mut r: *mut core::ffi::c_char = libc::strpbrk(
        (s as *mut core::ffi::c_char).cast_const(),
        (c"wo".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((r).is_null())) as i32) != 0));
    assert!(((((r) == (s.offset((4) as isize) as *mut core::ffi::c_char)) as i32) != 0));
    assert!(
        ((((libc::strpbrk(
            (s as *mut core::ffi::c_char).cast_const(),
            (c"xyz".as_ptr().cast_mut()).cast_const()
        ))
        .is_null()) as i32)
            != 0)
    );
    let mut buf: [core::ffi::c_char; 4] = [
        (('a' as i32) as core::ffi::c_char),
        (('b' as i32) as core::ffi::c_char),
        (('c' as i32) as core::ffi::c_char),
        (('\0' as i32) as core::ffi::c_char),
    ];
    assert!(
        ((((libc::strpbrk(
            (buf.as_mut_ptr()).cast_const(),
            (c"b".as_ptr().cast_mut()).cast_const()
        )) == (&mut buf[(1) as usize] as *mut core::ffi::c_char)) as i32)
            != 0)
    );
}
pub unsafe fn test_strcasecmp_15() {
    assert!(
        ((((libc::strcasecmp(
            (c"HELLO".as_ptr().cast_mut()).cast_const(),
            (c"hello".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcasecmp(
            (c"abc".as_ptr().cast_mut()).cast_const(),
            (c"abd".as_ptr().cast_mut()).cast_const()
        )) < (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcasecmp(
            (c"abd".as_ptr().cast_mut()).cast_const(),
            (c"abc".as_ptr().cast_mut()).cast_const()
        )) > (0)) as i32)
            != 0)
    );
    let mut p: *const core::ffi::c_char = (c"FOO".as_ptr().cast_mut()).cast_const();
    let mut q: *const core::ffi::c_char = (c"foo".as_ptr().cast_mut()).cast_const();
    assert!(((((libc::strcasecmp(p, q)) == (0)) as i32) != 0));
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
    (unsafe { test_strcspn_11() });
    (unsafe { test_strspn_12() });
    (unsafe { test_strstr_13() });
    (unsafe { test_strpbrk_14() });
    (unsafe { test_strcasecmp_15() });
    return 0;
}
