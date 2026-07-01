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
    let mut str: Vec<libc::c_char> = {
        let s = c"Hello, world!\n".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let file: [libc::c_char; 25] = libcc2rs::char_array(b"test_stdcopy_ostream.txt\0");
    let mut ofs: ::std::fs::File =
        ::std::fs::File::create(::std::ffi::CStr::from_ptr(file.as_ptr()).to_str().unwrap())
            .unwrap();
    {
        let __start = str.as_mut_ptr() as *const u8;
        let __end = str.as_mut_ptr().add(str.len() - 1) as *const u8;
        let __len = __end.offset_from(__start) as usize;
        ofs.write_all(::std::slice::from_raw_parts(__start, __len));
        ofs.try_clone().unwrap()
    };
    libc::unlink(file.as_ptr());
    return 0;
}
