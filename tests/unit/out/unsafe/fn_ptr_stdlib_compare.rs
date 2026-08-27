extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_alternative_fread_0(
    mut p: *mut libc::c_char,
    mut n: usize,
    mut m: usize,
    mut f: *mut ::libc::c_void,
) -> usize {
    return 22_usize;
}
pub unsafe fn my_alternative_fwrite_1(
    mut p: *const libc::c_char,
    mut n: usize,
    mut m: usize,
    mut f: *mut ::libc::c_void,
) -> usize {
    return 33_usize;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn1: Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize> =
        Some(libcc2rs::fread_unsafe);
    assert!(((fn1) == (Some(libcc2rs::fread_unsafe))));
    assert!(!((fn1).is_none()));
    let mut fn2: Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
            Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
        >(Some(libcc2rs::fread_unsafe));
    assert!(
        ((fn1)
            == (std::mem::transmute::<
                Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
                Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
            >(fn2)))
    );
    let mut f3: Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*mut libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        >(Some(my_alternative_fread_0));
    assert!(
        ((unsafe { (f3).unwrap()(std::ptr::null_mut(), 0_usize, 0_usize, std::ptr::null_mut(),) })
            == (22_usize))
    );
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/zero".as_ptr(), c"rb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
        {
            let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 16]>() {
                *byte_0.offset(offset as isize) = (('X' as libc::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = libcc2rs::fread_unsafe(
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            1_usize,
            10_usize,
            stream,
        );
        assert!(((n) == (10_usize)));
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((buf[(i) as usize] as i32) == (0)));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            assert!(((buf[(i) as usize] as i32) == (('X' as libc::c_char) as i32)));
            i.prefix_inc();
        }
        libc::fclose(stream);
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/zero".as_ptr(), c"rb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
        {
            let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 16]>() {
                *byte_0.offset(offset as isize) = (('X' as libc::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = (unsafe {
            (fn1).unwrap()(
                (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
                1_usize,
                10_usize,
                stream,
            )
        });
        assert!(((n) == (10_usize)));
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((buf[(i) as usize] as i32) == (0)));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            assert!(((buf[(i) as usize] as i32) == (('X' as libc::c_char) as i32)));
            i.prefix_inc();
        }
        libc::fclose(stream);
    }
    let mut gn1: Option<
        unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize,
    > = Some(libcc2rs::fwrite_unsafe);
    assert!(((gn1) == (Some(libcc2rs::fwrite_unsafe))));
    assert!(!((gn1).is_none()));
    let mut gn2: Option<
        unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize,
    > = std::mem::transmute::<
        Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        Option<unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
    >(Some(libcc2rs::fwrite_unsafe));
    assert!(
        ((gn1)
            == (std::mem::transmute::<
                Option<unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
                Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
            >(gn2)))
    );
    let mut g3: Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*const libc::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        >(Some(my_alternative_fwrite_1));
    assert!(
        ((unsafe { (g3).unwrap()(std::ptr::null(), 0_usize, 0_usize, std::ptr::null_mut(),) })
            == (33_usize))
    );
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/null".as_ptr(), c"wb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [libc::c_char; 10] = [(0 as libc::c_char); 10];
        {
            let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 10]>() {
                *byte_0.offset(offset as isize) = (('Y' as libc::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = libcc2rs::fwrite_unsafe(
            (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void),
            1_usize,
            10_usize,
            stream,
        );
        assert!(((n) == (10_usize)));
        libc::fclose(stream);
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/null".as_ptr(), c"wb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [libc::c_char; 10] = [(0 as libc::c_char); 10];
        {
            let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[libc::c_char; 10]>() {
                *byte_0.offset(offset as isize) = (('Y' as libc::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = (unsafe {
            (gn1).unwrap()(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void),
                1_usize,
                10_usize,
                stream,
            )
        });
        assert!(((n) == (10_usize)));
        libc::fclose(stream);
    }
    return 0;
}
