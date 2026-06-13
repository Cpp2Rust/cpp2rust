extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let str: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"Hello, world!\n")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<core::ffi::c_char>>(),
    ));
    let file: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(Box::from(
        libcc2rs::char_array(b"test_stdcopy_ostream.txt\0"),
    )));
    let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create((file.as_pointer() as Ptr<core::ffi::c_char>).to_string())
            .expect("Failed to open file"),
    ));
    {
        (*ofs.borrow_mut()).write_all(
            (str.as_pointer() as Ptr<core::ffi::c_char>)
                .reinterpret_cast::<u8>()
                .slice_until(
                    &(str.as_pointer() as Ptr<core::ffi::c_char>)
                        .to_last()
                        .reinterpret_cast::<u8>(),
                )
                .as_slice(),
        );
        (*ofs.borrow_mut()).try_clone().unwrap()
    };
    libc::unlink((file.as_pointer() as Ptr<core::ffi::c_char>));
    return 0;
}
