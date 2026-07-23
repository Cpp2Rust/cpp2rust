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
    let s: Value<i32> = Rc::new(RefCell::new({
        let __family = match libc::AF_INET {
            ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
            ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
            ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
            __d => panic!("socket: unsupported domain {__d}"),
        };
        let __flags = nix::sys::socket::SockFlag::from_bits_truncate(libc::SOCK_STREAM);
        let __ty = match libc::SOCK_STREAM & !nix::sys::socket::SockFlag::all().bits() {
            ::libc::SOCK_STREAM => nix::sys::socket::SockType::Stream,
            ::libc::SOCK_DGRAM => nix::sys::socket::SockType::Datagram,
            __t => panic!("socket: unsupported type {__t}"),
        };
        let __proto = match 0 {
            0 => None,
            ::libc::IPPROTO_TCP => Some(nix::sys::socket::SockProtocol::Tcp),
            ::libc::IPPROTO_UDP => Some(nix::sys::socket::SockProtocol::Udp),
            __p => panic!("socket: unsupported protocol {__p}"),
        };
        match nix::sys::socket::socket(__family, __ty, __flags, __proto) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*s.borrow()) >= 0) as i32) != 0));
    let on: Value<i32> = Rc::new(RefCell::new(1));
    assert!(
        ((({
            let __a0 = (*s.borrow());
            let __a1 = libc::SOL_SOCKET;
            let __a2 = libc::SO_KEEPALIVE;
            let __a3 = ((on.as_pointer()) as Ptr<i32>).to_any();
            libcc2rs::setsockopt_refcount(__a0, __a1, __a2, __a3)
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __a0 = (*s.borrow());
            let __a1 = libc::IPPROTO_TCP;
            let __a2 = libc::TCP_NODELAY;
            let __a3 = ((on.as_pointer()) as Ptr<i32>).to_any();
            libcc2rs::setsockopt_refcount(__a0, __a1, __a2, __a3)
        } == 0) as i32)
            != 0)
    );
    let err: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let len: Value<u32> = Rc::new(RefCell::new((::std::mem::size_of::<i32>() as u32)));
    assert!(
        (((match (libc::SOL_SOCKET, libc::SO_ERROR) {
            (::libc::SOL_SOCKET, ::libc::SO_ERROR) => {
                match FdRegistry::with_fd((*s.borrow()), |__fd| {
                    nix::sys::socket::getsockopt(&__fd, nix::sys::socket::sockopt::SocketError)
                }) {
                    Ok(__err) => {
                        ((err.as_pointer()) as Ptr<i32>)
                            .to_any()
                            .reinterpret_cast::<i32>()
                            .write(__err);
                        (len.as_pointer()).write(::std::mem::size_of::<i32>() as u32);
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            }
            (__l, __o) => panic!(
                "getsockopt: unsupported option (level={}, optname={})",
                __l, __o
            ),
        } == 0) as i32)
            != 0)
    );
    assert!(((((*err.borrow()) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*s.borrow())) == 0) as i32) != 0));
    return 0;
}
