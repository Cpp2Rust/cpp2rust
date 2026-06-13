extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_alternative_fread_0(
    mut p: *mut core::ffi::c_char,
    mut n: usize,
    mut m: usize,
    mut f: *mut ::libc::c_void,
) -> usize {
    return 22_usize;
}
pub unsafe fn my_alternative_fwrite_1(
    mut p: *const core::ffi::c_char,
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
    let mut fn2: Option<
        unsafe fn(*mut core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize,
    > = std::mem::transmute::<
        Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        Option<unsafe fn(*mut core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
    >(Some(libcc2rs::fread_unsafe));
    assert!(
        ((fn1)
            == (std::mem::transmute::<
                Option<
                    unsafe fn(*mut core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize,
                >,
                Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
            >(fn2)))
    );
    let mut f3: Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*mut core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*mut ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        >(Some(my_alternative_fread_0));
    assert!(
        ((unsafe {
            let _arg0: *mut ::libc::c_void = std::ptr::null_mut();
            let _arg3: *mut ::libc::FILE = std::ptr::null_mut();
            (f3).unwrap()(_arg0, 0_usize, 0_usize, _arg3)
        }) == (22_usize))
    );
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/zero".as_ptr(), c"rb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [core::ffi::c_char; 16] = [(0 as core::ffi::c_char); 16];
        {
            let byte_0 =
                (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[core::ffi::c_char; 16]>() {
                *byte_0.offset(offset as isize) = (('X' as core::ffi::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = libcc2rs::fread_unsafe(
            (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void),
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
            assert!(((buf[(i) as usize] as i32) == (('X' as core::ffi::c_char) as i32)));
            i.prefix_inc();
        }
        libc::fclose(stream);
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/zero".as_ptr(), c"rb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [core::ffi::c_char; 16] = [(0 as core::ffi::c_char); 16];
        {
            let byte_0 =
                (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[core::ffi::c_char; 16]>() {
                *byte_0.offset(offset as isize) = (('X' as core::ffi::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = (unsafe {
            let _arg0: *mut ::libc::c_void =
                (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void);
            let _arg3: *mut ::libc::FILE = stream;
            (fn1).unwrap()(_arg0, 1_usize, 10_usize, _arg3)
        });
        assert!(((n) == (10_usize)));
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((buf[(i) as usize] as i32) == (0)));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            assert!(((buf[(i) as usize] as i32) == (('X' as core::ffi::c_char) as i32)));
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
        unsafe fn(*const core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize,
    > = std::mem::transmute::<
        Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        Option<unsafe fn(*const core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
    >(Some(libcc2rs::fwrite_unsafe));
    assert!(
        ((gn1)
            == (std::mem::transmute::<
                Option<
                    unsafe fn(*const core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize,
                >,
                Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
            >(gn2)))
    );
    let mut g3: Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize> =
        std::mem::transmute::<
            Option<unsafe fn(*const core::ffi::c_char, usize, usize, *mut ::libc::c_void) -> usize>,
            Option<unsafe fn(*const ::libc::c_void, usize, usize, *mut ::libc::FILE) -> usize>,
        >(Some(my_alternative_fwrite_1));
    assert!(
        ((unsafe {
            let _arg0: *const ::libc::c_void = std::ptr::null();
            let _arg3: *mut ::libc::FILE = std::ptr::null_mut();
            (g3).unwrap()(_arg0, 0_usize, 0_usize, _arg3)
        }) == (33_usize))
    );
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: *mut ::libc::FILE = libc::fopen(c"/dev/null".as_ptr(), c"wb".as_ptr());
        assert!(!((stream).is_null()));
        let mut buf: [core::ffi::c_char; 10] = [(0 as core::ffi::c_char); 10];
        {
            let byte_0 =
                (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[core::ffi::c_char; 10]>() {
                *byte_0.offset(offset as isize) = (('Y' as core::ffi::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = libcc2rs::fwrite_unsafe(
            (buf.as_mut_ptr() as *const core::ffi::c_char as *const ::libc::c_void),
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
        let mut buf: [core::ffi::c_char; 10] = [(0 as core::ffi::c_char); 10];
        {
            let byte_0 =
                (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[core::ffi::c_char; 10]>() {
                *byte_0.offset(offset as isize) = (('Y' as core::ffi::c_char) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut core::ffi::c_char as *mut ::libc::c_void)
        };
        let mut n: usize = (unsafe {
            let _arg0: *const ::libc::c_void =
                (buf.as_mut_ptr() as *const core::ffi::c_char as *const ::libc::c_void);
            let _arg3: *mut ::libc::FILE = stream;
            (gn1).unwrap()(_arg0, 1_usize, 10_usize, _arg3)
        });
        assert!(((n) == (10_usize)));
        libc::fclose(stream);
    }
    return 0;
}
