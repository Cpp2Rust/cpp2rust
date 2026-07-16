extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_fputc_fputs_0() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_puts.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fputc(('A' as i32), fp)) == ('A' as i32)) as i32) != 0));
    assert!(
        ((((libc::fputs((c"BCD\n".as_ptr().cast_mut()).cast_const(), fp)) >= (0)) as i32) != 0)
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut buf: [libc::c_char; 16] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libcc2rs::fread_unsafe(
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            1_usize,
            16_usize,
            fp
        )) == (5_usize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"ABCD\n".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                5_usize as usize,
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
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_puts_1() {
    assert!(
        ((((libc::puts((c"hello from puts".as_ptr().cast_mut()).cast_const())) >= (0)) as i32)
            != 0)
    );
}
pub unsafe fn test_fgets_getc_2() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_gets.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        ((((libc::fputs((c"line1\nline2\n".as_ptr().cast_mut()).cast_const(), fp)) >= (0)) as i32)
            != 0)
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut buf: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    assert!((((!((libc::fgets(buf.as_mut_ptr(), 8, fp)).is_null())) as i32) != 0));
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                7_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"line1\n".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                7_usize as usize,
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
    assert!(((((libc::fgetc(fp)) == ('l' as i32)) as i32) != 0));
    assert!((((!((libc::fgets(buf.as_mut_ptr(), 4, fp)).is_null())) as i32) != 0));
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"ine".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
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
    assert!((((!((libc::fgets(buf.as_mut_ptr(), 8, fp)).is_null())) as i32) != 0));
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                3_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"2\n".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                3_usize as usize,
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
    assert!(((((libc::fgets(buf.as_mut_ptr(), 8, fp)).is_null()) as i32) != 0));
    assert!(((((libc::fgetc(fp)) == (-1_i32)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_freopen_3() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_reopen.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        ((((libc::fputs((c"hello".as_ptr().cast_mut()).cast_const(), fp)) >= (0)) as i32) != 0)
    );
    let mut fp2: *mut ::libc::FILE =
        libc::freopen(path, (c"rb".as_ptr().cast_mut()).cast_const(), fp);
    assert!((((!((fp2).is_null())) as i32) != 0));
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libcc2rs::fread_unsafe(
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            1_usize,
            8_usize,
            fp2
        )) == (5_usize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"hello".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                5_usize as usize,
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
    assert!(((((libc::fclose(fp2)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_fseeko_4() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_seek.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        ((((libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp)) >= (0)) as i32)
            != 0)
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fseeko(fp, 6_i64 as ::libc::off_t, 0)) == (0)) as i32) != 0));
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libcc2rs::fread_unsafe(
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            1_usize,
            5_usize,
            fp
        )) == (5_usize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                5_usize as usize,
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
    assert!(((((libc::fseeko(fp, (-5_i32 as i64) as ::libc::off_t, 2)) == (0)) as i32) != 0));
    assert!(((((libc::fgetc(fp)) == ('w' as i32)) as i32) != 0));
    assert!(((((libc::fseeko(fp, 1_i64 as ::libc::off_t, 1)) == (0)) as i32) != 0));
    assert!(((((libc::fgetc(fp)) == ('r' as i32)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub unsafe fn test_rename_5() {
    let mut from: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_from.tmp".as_ptr().cast_mut()).cast_const();
    let mut to: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_to.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(from, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fputs((c"data".as_ptr().cast_mut()).cast_const(), fp)) >= (0)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::rename(from, to)) == (0)) as i32) != 0));
    assert!(
        ((((libc::fopen(from, (c"rb".as_ptr().cast_mut()).cast_const())).is_null()) as i32) != 0)
    );
    fp = libc::fopen(to, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::rename(from, to)) == (-1_i32)) as i32) != 0));
    assert!(((((libc::unlink(to)) == (0)) as i32) != 0));
}
pub unsafe fn test_setvbuf_6() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_stdio_nofd_vbuf.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::setvbuf(fp, std::ptr::null_mut(), 2, 0_usize)) == (0)) as i32) != 0));
    assert!(((((libc::fputs((c"x".as_ptr().cast_mut()).cast_const(), fp)) >= (0)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_fputc_fputs_0() });
    (unsafe { test_puts_1() });
    (unsafe { test_fgets_getc_2() });
    (unsafe { test_freopen_3() });
    (unsafe { test_fseeko_4() });
    (unsafe { test_rename_5() });
    (unsafe { test_setvbuf_6() });
    return 0;
}
