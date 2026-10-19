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
    let mut str: Vec<libc::c_char> = {
        let s = c"Hello, world!\n".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let file: [libc::c_char; 25] = std::mem::transmute(*b"test_stdcopy_ostream.txt\0");
    {
        let mut ofs: ::std::fs::File =
            ::std::fs::File::create(::std::ffi::CStr::from_ptr(file.as_ptr()).to_str().unwrap())
                .unwrap();
        {
            let mut __bytes: Vec<u8> = Vec::new();
            let mut __it = UnsafeRandomAccessIterator::Vec(str.as_mut_ptr());
            while __it != UnsafeRandomAccessIterator::Vec(str.as_mut_ptr().add(str.len() - 1)) {
                __bytes.push(*__it.iter_deref() as u8);
                __it.iter_inc();
            }
            ofs.write_all(&__bytes);
            ofs.try_clone().unwrap()
        };
    }
    libc::unlink(file.as_ptr());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
