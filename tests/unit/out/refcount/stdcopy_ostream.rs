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
    let str: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"Hello, world!\n")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    let file: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"test_stdcopy_ostream.txt\0")));
    let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
        ::std::fs::File::create((file.as_pointer() as Ptr<u8>).to_string())
            .expect("Failed to open file"),
    ));
    {
        (*ofs.borrow_mut()).write_all(
            (str.as_pointer() as Ptr<u8>)
                .reinterpret_cast::<u8>()
                .slice_until(
                    &(str.as_pointer() as Ptr<u8>)
                        .to_last()
                        .reinterpret_cast::<u8>(),
                )
                .as_slice(),
        );
        (*ofs.borrow_mut()).try_clone().unwrap()
    };
    libc::unlink((file.as_pointer() as Ptr<u8>));
    return 0;
}
