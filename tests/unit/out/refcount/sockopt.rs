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
            let __res = match (1, 9) {
                (::libc::IPPROTO_TCP, ::libc::TCP_NODELAY) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpNoDelay,
                            &__v,
                        )
                    })
                }
                (::libc::SOL_SOCKET, ::libc::SO_KEEPALIVE) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::KeepAlive,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_TCP, ::libc::TCP_KEEPINTVL) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<u32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpKeepInterval,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_TCP, ::libc::TCP_KEEPCNT) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<u32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpKeepCount,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_IP, ::libc::IP_TOS) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::Ipv4Tos,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_IPV6, ::libc::IPV6_TCLASS) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::Ipv6TClass,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::IPPROTO_TCP, ::libc::TCP_KEEPIDLE) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<u32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpKeepIdle,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::SOL_SOCKET, ::libc::SO_BINDTODEVICE) => {
                    let __v = ::std::ffi::OsString::from(
                        ((on.as_pointer()) as Ptr<i32>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .to_rust_string(),
                    );
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::BindToDevice,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::IPPROTO_IP, ::libc::IP_BIND_ADDRESS_NO_PORT) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::IpBindAddressNoPort,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::IPPROTO_TCP, ::libc::TCP_FASTOPEN_CONNECT) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpFastOpenConnect,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::SOL_SOCKET, ::libc::SO_PRIORITY) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::Priority,
                            &__v,
                        )
                    })
                }
                (__l, __o) => panic!(
                    "setsockopt: unsupported option (level={}, optname={})",
                    __l, __o
                ),
            };
            match __res {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __res = match (libc::IPPROTO_TCP, 1) {
                (::libc::IPPROTO_TCP, ::libc::TCP_NODELAY) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpNoDelay,
                            &__v,
                        )
                    })
                }
                (::libc::SOL_SOCKET, ::libc::SO_KEEPALIVE) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::KeepAlive,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_TCP, ::libc::TCP_KEEPINTVL) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<u32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpKeepInterval,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_TCP, ::libc::TCP_KEEPCNT) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<u32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpKeepCount,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_IP, ::libc::IP_TOS) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::Ipv4Tos,
                            &__v,
                        )
                    })
                }
                (::libc::IPPROTO_IPV6, ::libc::IPV6_TCLASS) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::Ipv6TClass,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::IPPROTO_TCP, ::libc::TCP_KEEPIDLE) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<u32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpKeepIdle,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::SOL_SOCKET, ::libc::SO_BINDTODEVICE) => {
                    let __v = ::std::ffi::OsString::from(
                        ((on.as_pointer()) as Ptr<i32>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .to_rust_string(),
                    );
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::BindToDevice,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::IPPROTO_IP, ::libc::IP_BIND_ADDRESS_NO_PORT) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::IpBindAddressNoPort,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::IPPROTO_TCP, ::libc::TCP_FASTOPEN_CONNECT) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read()
                        != 0;
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::TcpFastOpenConnect,
                            &__v,
                        )
                    })
                }
                #[cfg(target_os = "linux")]
                (::libc::SOL_SOCKET, ::libc::SO_PRIORITY) => {
                    let __v = ((on.as_pointer()) as Ptr<i32>)
                        .to_any()
                        .reinterpret_cast::<i32>()
                        .read();
                    FdRegistry::with_fd((*s.borrow()), |__fd| {
                        nix::sys::socket::setsockopt(
                            &__fd,
                            nix::sys::socket::sockopt::Priority,
                            &__v,
                        )
                    })
                }
                (__l, __o) => panic!(
                    "setsockopt: unsupported option (level={}, optname={})",
                    __l, __o
                ),
            };
            match __res {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    let err: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let len: Value<u32> = Rc::new(RefCell::new((::std::mem::size_of::<i32>() as u32)));
    assert!(
        (((match (1, 4) {
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
