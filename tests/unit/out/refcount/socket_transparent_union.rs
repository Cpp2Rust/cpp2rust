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
    let fd: Value<i32> = Rc::new(RefCell::new(0));
    let ssloc: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    let slen: Value<u32> = Rc::new(RefCell::new((128usize as u32)));
    assert!(
        (((match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>((*fd.borrow()))
        {
            Ok(__ss) => {
                libcc2rs::encode_sockaddr(
                    &__ss,
                    &(ssloc.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
                    &(slen.as_pointer()),
                );
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
    let sin: Value<libcc2rs::SockaddrIn> = Rc::new(RefCell::new(Default::default()));
    let inlen: Value<u32> = Rc::new(RefCell::new((16usize as u32)));
    assert!(
        (((match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>((*fd.borrow()))
        {
            Ok(__ss) => {
                libcc2rs::encode_sockaddr(
                    &__ss,
                    &(sin.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
                    &(inlen.as_pointer()),
                );
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
    return 0;
}
