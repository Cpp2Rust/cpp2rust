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
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            Ptr::from_string_literal(b"/dev/null")
                .to_rust_string()
                .as_str(),
            nix::fcntl::OFlag::from_bits_retain(0),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    FdRegistry::close((*fd.borrow()));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let n: Value<isize> = Rc::new(RefCell::new(
        match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(::std::mem::size_of::<[u8; 4]>(), |__buf| {
                    nix::unistd::read(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
    ));
    return if ((((*n.borrow()) == (-1_i32 as isize)) as i32) != 0) {
        0
    } else {
        1
    };
}
