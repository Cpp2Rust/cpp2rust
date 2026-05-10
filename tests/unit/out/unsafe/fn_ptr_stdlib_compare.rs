extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_alternative_fread_0(
    mut p: *mut u8,
    mut n: u64,
    mut m: u64,
    mut f: *mut ::libc::c_void,
) -> u64 {
    return 22_u64;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn1: Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
        Some(libcc2rs::fread_unsafe);
    assert!(((fn1) == (Some(libcc2rs::fread_unsafe))));
    assert!(!((fn1).is_none()));
    let mut fn2: Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64> =
        std::mem::transmute::<
            Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
            Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
        >(Some(libcc2rs::fread_unsafe));
    assert!(
        ((fn1)
            == (std::mem::transmute::<
                Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
                Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
            >(fn2)))
    );
    let mut f3: Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64> =
        std::mem::transmute::<
            Option<unsafe fn(*mut u8, u64, u64, *mut ::libc::c_void) -> u64>,
            Option<unsafe fn(*mut ::libc::c_void, u64, u64, *mut ::std::fs::File) -> u64>,
        >(Some(my_alternative_fread_0));
    assert!(
        ((unsafe {
            let _arg0: *mut ::libc::c_void = std::ptr::null_mut();
            let _arg1: u64 = 0_u64;
            let _arg2: u64 = 0_u64;
            let _arg3: *mut ::std::fs::File = std::ptr::null_mut();
            (f3).unwrap()(_arg0, _arg1, _arg2, _arg3)
        }) == (22_u64))
    );
    'loop_: loop {
        let mut stream: *mut ::std::fs::File =
            match std::ffi::CStr::from_ptr(b"rb\0".as_ptr() as *const i8)
                .to_str()
                .expect("invalid c-string")
            {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(
                        std::ffi::CStr::from_ptr(b"/dev/zero\0".as_ptr() as *const i8)
                            .to_str()
                            .expect("invalid c-string"),
                    )
                    .ok()
                    .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(
                        std::ffi::CStr::from_ptr(b"/dev/zero\0".as_ptr() as *const i8)
                            .to_str()
                            .expect("invalid c-string"),
                    )
                    .ok()
                    .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
                _ => panic!("unsupported mode"),
            };
        assert!(!((stream).is_null()));
        let mut buf: [u8; 16] = [0_u8; 16];
        {
            let byte_0 = (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[u8; 16]>() as u64 {
                *byte_0.offset(offset as isize) = (('X' as u8) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
        };
        let mut n: u64 = unsafe {
            libcc2rs::fread_unsafe(
                (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut ::std::ffi::c_void,
                1_u64,
                10_u64,
                stream,
            )
        };
        assert!(((n) == (10_u64)));
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((buf[(i) as usize] as i32) == (0)));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            assert!(((buf[(i) as usize] as i32) == (('X' as u8) as i32)));
            i.prefix_inc();
        }
        {
            Box::from_raw(stream);
            0
        };
        if !(0 != 0) {
            break;
        }
    }
    'loop_: loop {
        let mut stream: *mut ::std::fs::File =
            match std::ffi::CStr::from_ptr(b"rb\0".as_ptr() as *const i8)
                .to_str()
                .expect("invalid c-string")
            {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(
                        std::ffi::CStr::from_ptr(b"/dev/zero\0".as_ptr() as *const i8)
                            .to_str()
                            .expect("invalid c-string"),
                    )
                    .ok()
                    .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(
                        std::ffi::CStr::from_ptr(b"/dev/zero\0".as_ptr() as *const i8)
                            .to_str()
                            .expect("invalid c-string"),
                    )
                    .ok()
                    .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
                _ => panic!("unsupported mode"),
            };
        assert!(!((stream).is_null()));
        let mut buf: [u8; 16] = [0_u8; 16];
        {
            let byte_0 = (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut u8;
            for offset in 0..::std::mem::size_of::<[u8; 16]>() as u64 {
                *byte_0.offset(offset as isize) = (('X' as u8) as i32) as u8;
            }
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
        };
        let mut n: u64 = (unsafe {
            let _arg0: *mut ::libc::c_void = (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void);
            let _arg1: u64 = 1_u64;
            let _arg2: u64 = 10_u64;
            let _arg3: *mut ::std::fs::File = stream;
            (fn1).unwrap()(_arg0, _arg1, _arg2, _arg3)
        });
        assert!(((n) == (10_u64)));
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((buf[(i) as usize] as i32) == (0)));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while ((i) < (16)) {
            assert!(((buf[(i) as usize] as i32) == (('X' as u8) as i32)));
            i.prefix_inc();
        }
        {
            Box::from_raw(stream);
            0
        };
        if !(0 != 0) {
            break;
        }
    }
    return 0;
}
