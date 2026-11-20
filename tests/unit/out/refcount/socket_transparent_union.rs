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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut fd: i32 = 0;
    let ssloc: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    let slen: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_0 = { (ssloc.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_1 = { (slen.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_0, &__tmp_1);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == -1_i32)
    );
    let sin: Value<libcc2rs::SockaddrIn> = Rc::new(RefCell::new(Default::default()));
    let inlen: Value<libc::socklen_t> = Rc::new(RefCell::new((16usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_2 = { (sin.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_3 = { (inlen.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_2, &__tmp_3);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == -1_i32)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
