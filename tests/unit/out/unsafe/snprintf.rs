extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 32] = [(0 as libc::c_char); 32];
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"x=%d y=%u".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (-3_i32),
                (7_u32),
            )
        }) == (8)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"x=-3 y=7".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                4_usize as usize,
                (c"%s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (c"hello".as_ptr().cast_mut()),
            )
        }) == (5)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"hel".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%05d|%x|%X".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (42),
                (255),
                (255),
            )
        }) == (11)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"00042|ff|FF".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%.2f".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (3.14159E+0),
            )
        }) == (4)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"3.14".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%-6s|".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (c"ab".as_ptr().cast_mut()),
            )
        }) == (7)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"ab    |".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%c%%".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (65),
            )
        }) == (2)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"A%".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%+d % d".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (5),
                (5),
            )
        }) == (5)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"+5  5".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%ld %lu %zu".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (-1_i64),
                (1_u64),
                (9 as usize),
            )
        }) == (6)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"-1 1 9".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%e".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (1.2345678E+3),
            )
        }) == (12)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"1.234568e+03".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%g".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (1.234567E+6),
            )
        }) == (11)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"1.23457e+06".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    buf[(0) as usize] = (('Z' as i32) as libc::c_char);
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                0_usize as usize,
                (c"%d".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (123),
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(((((buf[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0));
    let mut fmt: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    fmt[(0) as usize] = (('%' as i32) as libc::c_char);
    fmt[(1) as usize] = (('5' as i32) as libc::c_char);
    fmt[(2) as usize] = (('.' as i32) as libc::c_char);
    fmt[(3) as usize] = (('1' as i32) as libc::c_char);
    fmt[(4) as usize] = (('f' as i32) as libc::c_char);
    fmt[(5) as usize] = (0 as libc::c_char);
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (fmt.as_mut_ptr()).cast_const() as *const libc::c_char,
                (3.26E+0),
            )
        }) == (5)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"  3.3".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    return 0;
}
