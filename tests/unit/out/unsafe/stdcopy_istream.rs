extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut data_0: std::cell::LazyCell<[u8; 3]> =
    std::cell::LazyCell::new(|| unsafe { [0_u8, 127_u8, 255_u8] });
pub unsafe fn check_1(s: *const Vec<libc::c_char>) {
    assert!((((*s).len() - 1) == (::std::mem::size_of::<[u8; 3]>())));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (::std::mem::size_of::<[u8; 3]>())) {
        assert!(
            ((((&(*s))[(i as usize)] as u8) as i32)
                == ((*std::cell::LazyCell::force_mut(&mut *&raw mut data_0))[(i) as usize] as i32))
        );
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let file: [libc::c_char; 25] = std::mem::transmute(*b"test_stdcopy_istream.txt\0");
    let mut fd: i32 = (unsafe {
        libc::open(
            file.as_ptr() as *const i8,
            (((::libc::O_WRONLY) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (384),
        )
    });
    assert!(((fd) >= (0)));
    assert!(
        ((libc::write(
            fd,
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut data_0)).as_ptr()
                as *const ::libc::c_void),
            ::std::mem::size_of::<[u8; 3]>()
        ) as usize)
            == (::std::mem::size_of::<[u8; 3]>()))
    );
    assert!(((libc::close(fd)) == (0)));
    {
        let mut f: libcc2rs::CppStream =
            ::std::fs::File::open(::std::ffi::CStr::from_ptr(file.as_ptr()).to_str().unwrap())
                .map(libcc2rs::CppStream::from)
                .unwrap();
        let mut s: Vec<libc::c_char> = {
            let mut __bytes: Vec<u8> = Vec::new();
            (*(&mut f as *mut libcc2rs::CppStream))
                .read_to_end(&mut __bytes)
                .expect("couldn't read the file");
            let mut __buf: Vec<libc::c_char> = __bytes.iter().map(|&b| b as libc::c_char).collect();
            __buf.push(0);
            __buf
        };
        (unsafe { check_1(&s) });
    }
    {
        let mut f: libcc2rs::CppStream =
            ::std::fs::File::open(::std::ffi::CStr::from_ptr(file.as_ptr()).to_str().unwrap())
                .map(libcc2rs::CppStream::from)
                .unwrap();
        let mut begin: *mut libcc2rs::CppStream = (&mut f as *mut libcc2rs::CppStream);
        let mut copy: *mut libcc2rs::CppStream = (begin).clone();
        let mut end: *mut libcc2rs::CppStream = std::ptr::null_mut();
        let mut s: Vec<libc::c_char> = {
            let mut __bytes: Vec<u8> = Vec::new();
            (*(copy).clone())
                .read_to_end(&mut __bytes)
                .expect("couldn't read the file");
            let mut __buf: Vec<libc::c_char> = __bytes.iter().map(|&b| b as libc::c_char).collect();
            __buf.push(0);
            __buf
        };
        (unsafe { check_1(&s) });
    }
    libc::unlink(file.as_ptr());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data_0);
}
