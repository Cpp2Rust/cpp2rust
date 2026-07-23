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
        b"/tmp/cpp2rust_termios_test.tmp",
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
    let tio: Value<libcc2rs::Termios> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| nix::sys::termios::tcgetattr(__fd)) {
            Ok(__t) => {
                (tio.as_pointer()).with_mut(|__dst| *__dst = Termios::from_libc(&__t.into()));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
    assert!(
        ((({
            let __action = match 0 {
                0 => nix::sys::termios::SetArg::TCSANOW,
                1 => nix::sys::termios::SetArg::TCSADRAIN,
                2 => nix::sys::termios::SetArg::TCSAFLUSH,
                __a => panic!("tcsetattr: unsupported action {__a}"),
            };
            let __t =
                nix::sys::termios::Termios::from((tio.as_pointer()).with(|__src| __src.to_libc()));
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::sys::termios::tcsetattr(__fd, __action, &__t)
            }) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == -1_i32) as i32)
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
