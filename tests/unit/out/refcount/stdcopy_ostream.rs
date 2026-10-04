extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let str: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"Hello, world!\n").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let file: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"test_stdcopy_ostream.txt\0",
    )));
    {
        let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
            ::std::fs::File::create((file.as_pointer() as Ptr<i8>).to_string())
                .expect("Failed to open file"),
        ));
        {
            (str.as_pointer() as Ptr<i8>).with_slice(
                (str.as_pointer() as Ptr<i8>).to_last().get_offset()
                    - (str.as_pointer() as Ptr<i8>).get_offset(),
                |__s| CChar::with_u8_slice(__s, |__b| (*ofs.borrow_mut()).write_all(__b)),
            );
            (*ofs.borrow_mut()).try_clone().unwrap()
        };
    }
    match nix::unistd::unlink((file.as_pointer() as Ptr<i8>).to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
