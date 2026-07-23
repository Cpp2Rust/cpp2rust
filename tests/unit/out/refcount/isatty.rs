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
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_isatty_test.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(
                ((::libc::O_RDWR | ::libc::O_CREAT) | ::libc::O_TRUNC),
            ),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*fd.borrow()) >= 0) as i32) != 0));
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| nix::unistd::isatty(__fd)) {
            Ok(__tty) => __tty as i32,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                0
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fd.borrow())) == 0) as i32) != 0));
    assert!(
        (((match nix::unistd::unlink((*path.borrow()).to_rust_string().as_str()) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    return 0;
}
