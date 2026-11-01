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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut str: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"Hello, world!\n".as_ptr(),
        (0..)
            .take_while(|&i| *c"Hello, world!\n".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let file: [libc::c_char; 25] = std::mem::transmute(*b"test_stdcopy_ostream.txt\0");
    {
        let mut ofs: ::std::fs::File =
            ::std::fs::File::create(::std::ffi::CStr::from_ptr(file.as_ptr()).to_str().unwrap())
                .unwrap();
        match ({ UnsafeRandomAccessIterator::Vec(str.as_mut_ptr()) }, {
            UnsafeRandomAccessIterator::Vec(str.as_mut_ptr().add(str.len() - 1))
        }) {
            (mut __tmp_0, mut __tmp_1) => {
                let mut __it = (__tmp_0).clone();
                let __out = &mut ofs;
                let mut __bytes: Vec<u8> = Vec::new();
                while __it != __tmp_1 {
                    __bytes.push(*__it.iter_deref() as u8);
                    __it.iter_inc();
                }
                __out.write_all(&__bytes);
                __out.try_clone().unwrap()
            }
        };
    }
    libc::unlink(file.as_ptr());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
