extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_memcpy_0() {
    let src: [libc::c_char; 6] = libcc2rs::char_array(b"hello\0");
    let mut dst: [libc::c_char; 6] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    let mut r: *mut ::libc::c_void = {
        if 6_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (src.as_ptr() as *const libc::c_char as *const ::libc::c_void),
                (dst.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
                6_usize as usize,
            )
        }
        (dst.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(((r) == (dst.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)));
    assert!(
        (((dst[(0) as usize] as i32) == (('h' as libc::c_char) as i32))
            && ((dst[(1) as usize] as i32) == (('e' as libc::c_char) as i32)))
            && ((dst[(2) as usize] as i32) == (('l' as libc::c_char) as i32))
    );
    assert!(
        (((dst[(3) as usize] as i32) == (('l' as libc::c_char) as i32))
            && ((dst[(4) as usize] as i32) == (('o' as libc::c_char) as i32)))
            && ((dst[(5) as usize] as i32) == (('\0' as libc::c_char) as i32))
    );
}
pub unsafe fn test_memset_1() {
    let mut buf: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    let mut r: *mut ::libc::c_void = {
        let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..4_usize {
            *byte_0.offset(offset as isize) = (('x' as libc::c_char) as i32) as u8;
        }
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(((r) == (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)));
    assert!(
        ((((buf[(0) as usize] as i32) == (('x' as libc::c_char) as i32))
            && ((buf[(1) as usize] as i32) == (('x' as libc::c_char) as i32)))
            && ((buf[(2) as usize] as i32) == (('x' as libc::c_char) as i32)))
            && ((buf[(3) as usize] as i32) == (('x' as libc::c_char) as i32))
    );
}
pub unsafe fn test_memcmp_2() {
    let a: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    let b: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    let c: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (9 as libc::c_char),
        (4 as libc::c_char),
    ];
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (a.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (b.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
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
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (a.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
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
        }) < (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (c.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (a.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
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
        }) > (0))
    );
}
pub unsafe fn test_memmove_3() {
    let mut buf: [libc::c_char; 6] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('d' as libc::c_char),
        ('e' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    let mut r: *mut ::libc::c_void = {
        if 4_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void),
                (buf.as_mut_ptr().offset((1) as isize) as *mut libc::c_char as *mut ::libc::c_void),
                4_usize as usize,
            )
        }
        (buf.as_mut_ptr().offset((1) as isize) as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((r) == (buf.as_mut_ptr().offset((1) as isize) as *mut libc::c_char
            as *mut ::libc::c_void))
    );
    assert!(
        (((buf[(0) as usize] as i32) == (('a' as libc::c_char) as i32))
            && ((buf[(1) as usize] as i32) == (('a' as libc::c_char) as i32)))
            && ((buf[(2) as usize] as i32) == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((buf[(3) as usize] as i32) == (('c' as libc::c_char) as i32))
            && ((buf[(4) as usize] as i32) == (('d' as libc::c_char) as i32)))
            && ((buf[(5) as usize] as i32) == (('\0' as libc::c_char) as i32))
    );
}
pub unsafe fn test_strchr_4() {
    let mut s: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char =
        (libc::strchr(s, (('w' as libc::c_char) as i32)) as *const libc::c_char);
    assert!(!((r).is_null()));
    assert!((((*r) as i32) == (('w' as libc::c_char) as i32)));
    assert!((libc::strchr(s, (('z' as libc::c_char) as i32)) as *const libc::c_char).is_null());
}
pub unsafe fn test_strlen_5() {
    assert!(((libc::strlen(c"".as_ptr())) == (0_usize)));
    assert!(((libc::strlen(c"hello".as_ptr())) == (5_usize)));
    assert!(((libc::strlen(c"hello world".as_ptr())) == (11_usize)));
}
pub unsafe fn test_strcmp_6() {
    assert!(((libc::strcmp(c"abc".as_ptr(), c"abc".as_ptr())) == (0)));
    assert!(((libc::strcmp(c"abc".as_ptr(), c"abd".as_ptr())) < (0)));
    assert!(((libc::strcmp(c"abd".as_ptr(), c"abc".as_ptr())) > (0)));
    let mut p: *const libc::c_char = c"abc".as_ptr();
    let mut q: *const libc::c_char = c"abd".as_ptr();
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    assert!(((libc::strcmp(p, p)) == (0)));
    assert!(((libc::strcmp(p, q)) < (0)));
    assert!(((libc::strcmp((buf.as_mut_ptr()).cast_const(), p)) == (0)));
}
pub unsafe fn test_strncmp_7() {
    assert!(((libc::strncmp(c"abcdef".as_ptr(), c"abcxyz".as_ptr(), 3_usize as usize)) == (0)));
    assert!(((libc::strncmp(c"abcdef".as_ptr(), c"abcxyz".as_ptr(), 4_usize as usize)) < (0)));
    assert!(((libc::strncmp(c"abcxyz".as_ptr(), c"abcdef".as_ptr(), 4_usize as usize)) > (0)));
    let mut p: *const libc::c_char = c"abcdef".as_ptr();
    let mut q: *const libc::c_char = c"abcxyz".as_ptr();
    let mut buf: [libc::c_char; 7] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('d' as libc::c_char),
        ('e' as libc::c_char),
        ('f' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    let mut n: usize = 3_usize;
    assert!(((libc::strncmp(p, q, n as usize)) == (0)));
    assert!(((libc::strncmp(p, q, (n).wrapping_add(1_usize) as usize)) < (0)));
    assert!(((libc::strncmp((buf.as_mut_ptr()).cast_const(), p, 6_usize as usize)) == (0)));
}
pub unsafe fn test_memchr_8() {
    let data: [libc::c_char; 4] = [
        (16 as libc::c_char),
        (32 as libc::c_char),
        (48 as libc::c_char),
        (64 as libc::c_char),
    ];
    let mut r: *const ::libc::c_void = libc::memchr(
        (data.as_ptr() as *const libc::c_char as *const ::libc::c_void) as *const ::libc::c_void,
        48,
        4_usize as usize,
    ) as *const ::libc::c_void;
    assert!(
        ((r) == ((&data[(2) as usize] as *const libc::c_char) as *const libc::c_char
            as *const ::libc::c_void))
    );
    assert!(
        (libc::memchr(
            (data.as_ptr() as *const libc::c_char as *const ::libc::c_void)
                as *const ::libc::c_void,
            153,
            4_usize as usize
        ) as *const ::libc::c_void)
            .is_null()
    );
    let mut p: *const ::libc::c_void =
        (data.as_ptr() as *const libc::c_char as *const ::libc::c_void);
    let mut n: usize = 4_usize;
    assert!(
        ((libc::memchr(p as *const ::libc::c_void, 16, n as usize) as *const ::libc::c_void)
            == (p))
    );
}
pub unsafe fn test_strrchr_9() {
    let mut s: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char =
        (libc::strrchr(s, (('l' as libc::c_char) as i32)) as *const libc::c_char);
    assert!(!((r).is_null()));
    assert!((((*r) as i32) == (('l' as libc::c_char) as i32)));
    assert!(((r) == (s.offset((9) as isize))));
    assert!((libc::strrchr(s, (('z' as libc::c_char) as i32)) as *const libc::c_char).is_null());
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('a' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    assert!(
        ((libc::strrchr(buf.as_mut_ptr(), (('a' as libc::c_char) as i32)))
            == (&mut buf[(2) as usize] as *mut libc::c_char))
    );
}
pub unsafe fn test_strdup_10() {
    let mut d: *mut libc::c_char = libcc2rs::strdup_unsafe(c"hello".as_ptr());
    assert!(!((d).is_null()));
    assert!(((libc::strcmp((d).cast_const(), c"hello".as_ptr())) == (0)));
    libcc2rs::free_unsafe((d as *mut libc::c_char as *mut ::libc::c_void));
    let mut p: *const libc::c_char = c"world".as_ptr();
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    let mut d2: *mut libc::c_char = libcc2rs::strdup_unsafe(p);
    assert!(!((d2).is_null()));
    assert!(((libc::strcmp((d2).cast_const(), p)) == (0)));
    libcc2rs::free_unsafe((d2 as *mut libc::c_char as *mut ::libc::c_void));
    let mut d3: *mut libc::c_char = libcc2rs::strdup_unsafe((buf.as_mut_ptr()).cast_const());
    assert!(!((d3).is_null()));
    assert!(((libc::strcmp((d3).cast_const(), (buf.as_mut_ptr()).cast_const())) == (0)));
    libcc2rs::free_unsafe((d3 as *mut libc::c_char as *mut ::libc::c_void));
}
pub unsafe fn test_strcspn_11() {
    assert!(((libc::strcspn(c"hello".as_ptr(), c"el".as_ptr())) == (1_usize)));
    assert!(((libc::strcspn(c"abc".as_ptr(), c"xyz".as_ptr())) == (3_usize)));
    assert!(((libc::strcspn(c"".as_ptr(), c"abc".as_ptr())) == (0_usize)));
    let mut s: *const libc::c_char = c"hello".as_ptr();
    let mut rej: *const libc::c_char = c"el".as_ptr();
    assert!(((libc::strcspn(s, rej)) == (1_usize)));
}
pub unsafe fn test_strspn_12() {
    assert!(((libc::strspn(c"hello".as_ptr(), c"hel".as_ptr())) == (4_usize)));
    assert!(((libc::strspn(c"abc".as_ptr(), c"xyz".as_ptr())) == (0_usize)));
    assert!(((libc::strspn(c"aaa".as_ptr(), c"a".as_ptr())) == (3_usize)));
    let mut s: *const libc::c_char = c"hello".as_ptr();
    let mut acc: *const libc::c_char = c"hel".as_ptr();
    assert!(((libc::strspn(s, acc)) == (4_usize)));
}
pub unsafe fn test_strstr_13() {
    let mut h: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char = (libc::strstr(h, c"world".as_ptr()) as *const libc::c_char);
    assert!(!((r).is_null()));
    assert!(((r) == (h.offset((6) as isize))));
    assert!((libc::strstr(h, c"xyz".as_ptr()) as *const libc::c_char).is_null());
    let mut buf: [libc::c_char; 6] = [
        ('h' as libc::c_char),
        ('e' as libc::c_char),
        ('l' as libc::c_char),
        ('l' as libc::c_char),
        ('o' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    assert!(
        ((libc::strstr(buf.as_mut_ptr(), c"ll".as_ptr()))
            == (&mut buf[(2) as usize] as *mut libc::c_char))
    );
}
pub unsafe fn test_strpbrk_14() {
    let mut s: *const libc::c_char = c"hello world".as_ptr();
    let mut r: *const libc::c_char = (libc::strpbrk(s, c"wo".as_ptr()) as *const libc::c_char);
    assert!(!((r).is_null()));
    assert!(((r) == (s.offset((4) as isize))));
    assert!((libc::strpbrk(s, c"xyz".as_ptr()) as *const libc::c_char).is_null());
    let mut buf: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    assert!(
        ((libc::strpbrk(buf.as_mut_ptr(), c"b".as_ptr()))
            == (&mut buf[(1) as usize] as *mut libc::c_char))
    );
}
pub unsafe fn test_strcasecmp_15() {
    assert!(((libc::strcasecmp(c"HELLO".as_ptr(), c"hello".as_ptr())) == (0)));
    assert!(((libc::strcasecmp(c"abc".as_ptr(), c"abd".as_ptr())) < (0)));
    assert!(((libc::strcasecmp(c"abd".as_ptr(), c"abc".as_ptr())) > (0)));
    let mut p: *const libc::c_char = c"FOO".as_ptr();
    let mut q: *const libc::c_char = c"foo".as_ptr();
    assert!(((libc::strcasecmp(p, q)) == (0)));
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
