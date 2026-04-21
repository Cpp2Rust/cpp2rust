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
    let mut str: Vec<u8> = {
        let s = b"Hello, world!\n\0".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut ofs: ::std::fs::File = ::std::fs::File::create(
        ::std::ffi::CStr::from_ptr(b"test.txt\0".as_ptr() as *const i8)
            .to_str()
            .unwrap(),
    )
    .unwrap();
    {
        let __start = str.as_mut_ptr() as *const u8;
        let __end = str.as_mut_ptr().add(str.len() - 1) as *const u8;
        let __len = __end.offset_from(__start) as usize;
        ofs.write_all(::std::slice::from_raw_parts(__start, __len));
        ofs.try_clone().unwrap()
    };
    return 0;
}
