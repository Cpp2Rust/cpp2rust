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
    return 0;
}
