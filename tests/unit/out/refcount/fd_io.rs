extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_open_read_write_0() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_rw.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(
                ((::libc::O_WRONLY | ::libc::O_CREAT) | ::libc::O_TRUNC),
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
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            Ptr::from_string_literal(b"hello world")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(11_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 11_isize) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fd.borrow())) == 0) as i32) != 0));
    (*fd.borrow_mut()) = {
        let __mode = match &[].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(::libc::O_RDONLY),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!(((((*fd.borrow()) >= 0) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 16]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(::std::mem::size_of::<[u8; 16]>(), |__buf| {
                    nix::unistd::read(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 11_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"hello world").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(::std::mem::size_of::<[u8; 16]>(), |__buf| {
                    nix::unistd::read(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0_isize) as i32)
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
}
pub fn test_pipe_1() {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(1) as usize], |__fd| {
            Ptr::from_string_literal(b"ab")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(2_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 2_isize) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
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
        } == 2_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"ab").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
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
        } == 0_isize) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
}
pub fn test_socket_listen_2() {
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
    assert!(
        (((match nix::sys::socket::Backlog::new(5) {
            Ok(__b) => match FdRegistry::with_fd((*s.borrow()), |__fd| nix::sys::socket::listen(
                &__fd, __b
            )) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            },
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*s.borrow())) == 0) as i32) != 0));
}
pub fn test_sockopt_3() {
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
}
pub fn test_lseek_4() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_lseek.tmp",
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
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            Ptr::from_string_literal(b"hello world")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(11_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 11_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let __whence = match 2 {
                0 => nix::unistd::Whence::SeekSet,
                1 => nix::unistd::Whence::SeekCur,
                2 => nix::unistd::Whence::SeekEnd,
                __w => panic!("lseek: unsupported whence {__w}"),
            };
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::unistd::lseek(__fd, 0_i64, __whence)
            }) {
                Ok(__off) => __off,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 11_i64) as i32)
            != 0)
    );
    assert!(
        ((({
            let __whence = match 0 {
                0 => nix::unistd::Whence::SeekSet,
                1 => nix::unistd::Whence::SeekCur,
                2 => nix::unistd::Whence::SeekEnd,
                __w => panic!("lseek: unsupported whence {__w}"),
            };
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::unistd::lseek(__fd, 6_i64, __whence)
            }) {
                Ok(__off) => __off,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 6_i64) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 16]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(::std::mem::size_of::<[u8; 16]>(), |__buf| {
                    nix::unistd::read(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 5_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"world").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
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
}
pub fn test_ftruncate_5() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_trunc.tmp",
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
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            Ptr::from_string_literal(b"hello world")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(11_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 11_isize) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| nix::unistd::ftruncate(__fd, 5_i64)) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __whence = match 2 {
                0 => nix::unistd::Whence::SeekSet,
                1 => nix::unistd::Whence::SeekCur,
                2 => nix::unistd::Whence::SeekEnd,
                __w => panic!("lseek: unsupported whence {__w}"),
            };
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::unistd::lseek(__fd, 0_i64, __whence)
            }) {
                Ok(__off) => __off,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 5_i64) as i32)
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
}
pub fn test_fstat_6() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_fstat.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(
                ((::libc::O_WRONLY | ::libc::O_CREAT) | ::libc::O_TRUNC),
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
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            Ptr::from_string_literal(b"hello")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(5_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 5_isize) as i32)
            != 0)
    );
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| nix::sys::stat::fstat(__fd)) {
            Ok(__s) => {
                (st.as_pointer()).with_mut(|__st| *__st = Stat::from_libc(&__s));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*st.borrow()).st_size.borrow()) == 5_i64) as i32) != 0));
    assert!((((((*(*st.borrow()).st_mode.borrow()) & 61440_u32) == 32768_u32) as i32) != 0));
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
}
pub fn test_isatty_7() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_tty.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain((::libc::O_RDONLY | ::libc::O_CREAT)),
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
}
pub fn test_tcgetattr_8() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_termios.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain((::libc::O_RDONLY | ::libc::O_CREAT)),
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
}
pub fn test_fcntl_9() {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    let flags: Value<i32> = Rc::new(RefCell::new({
        let __res = match 3 {
            ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
            }),
            ::libc::F_SETFL => {
                let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                })
            }
            ::libc::F_SETFD => {
                let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                })
            }
            __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
        };
        match __res {
            Ok(__r) => __r,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*flags.borrow()) >= 0) as i32) != 0));
    assert!((((((*flags.borrow()) & ::libc::O_NONBLOCK) == 0) as i32) != 0));
    assert!(
        ((({
            let __res = match 4 {
                ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                        &&[((*flags.borrow()) | ::libc::O_NONBLOCK).into()][0],
                    ));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                        &&[((*flags.borrow()) | ::libc::O_NONBLOCK).into()][0],
                    ));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                    })
                }
                __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
            };
            match __res {
                Ok(__r) => __r,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    (*flags.borrow_mut()) = {
        let __res = match 3 {
            ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
            }),
            ::libc::F_SETFL => {
                let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                })
            }
            ::libc::F_SETFD => {
                let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                })
            }
            __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
        };
        match __res {
            Ok(__r) => __r,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!((((((*flags.borrow()) & ::libc::O_NONBLOCK) != 0) as i32) != 0));
    let b: Value<u8> = <Value<u8>>::default();
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((b.as_pointer()) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(1_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(
        ((({
            let __res = match 2 {
                ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(1).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(1).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                    })
                }
                __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
            };
            match __res {
                Ok(__r) => __r,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
}
pub fn test_select_10() {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    let rset: Value<CFdSet> = Rc::new(RefCell::new(Default::default()));
    (rset.as_pointer()).with_mut(|__s| __s.zero());
    (rset.as_pointer()).with_mut(|__s| __s.set((*fds.borrow())[(0) as usize]));
    let tv: Value<libcc2rs::Timeval> = Rc::new(RefCell::new(Default::default()));
    (*(*tv.borrow()).tv_sec.borrow_mut()) = 0_i64;
    (*(*tv.borrow()).tv_usec.borrow_mut()) = 0_i64;
    assert!(
        ((({
            let __rp = (rset.as_pointer()).clone();
            let __wp = Ptr::<CFdSet>::null().clone();
            let __ep = Ptr::<CFdSet>::null().clone();
            let __tp = (tv.as_pointer()).clone();
            let __r_fds: Vec<i32> = match __rp.is_null() {
                true => Vec::new(),
                false => __rp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __w_fds: Vec<i32> = match __wp.is_null() {
                true => Vec::new(),
                false => __wp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __e_fds: Vec<i32> = match __ep.is_null() {
                true => Vec::new(),
                false => __ep.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let mut __wanted = Vec::new();
            __wanted.extend(&__r_fds);
            __wanted.extend(&__w_fds);
            __wanted.extend(&__e_fds);
            FdRegistry::with_fds(&__wanted, |__b| {
                let __rn = __r_fds.len();
                let __wn = __w_fds.len();
                let mut __rs = nix::sys::select::FdSet::new();
                let mut __ws = nix::sys::select::FdSet::new();
                let mut __es = nix::sys::select::FdSet::new();
                for __bfd in &__b[..__rn] {
                    __rs.insert(*__bfd);
                }
                for __bfd in &__b[__rn..__rn + __wn] {
                    __ws.insert(*__bfd);
                }
                for __bfd in &__b[__rn + __wn..] {
                    __es.insert(*__bfd);
                }
                let mut __tv = match __tp.is_null() {
                    true => None,
                    false => Some(__tp.with(|__t| {
                        nix::sys::time::TimeVal::new(
                            *__t.tv_sec.borrow() as _,
                            *__t.tv_usec.borrow() as _,
                        )
                    })),
                };
                match nix::sys::select::select(
                    ((*fds.borrow())[(0) as usize] + 1),
                    match __rp.is_null() {
                        true => None,
                        false => Some(&mut __rs),
                    },
                    match __wp.is_null() {
                        true => None,
                        false => Some(&mut __ws),
                    },
                    match __ep.is_null() {
                        true => None,
                        false => Some(&mut __es),
                    },
                    __tv.as_mut(),
                ) {
                    Ok(__n) => {
                        if !__rp.is_null() {
                            __rp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __r_fds.iter().zip(&__b[..__rn]) {
                                    if __rs.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__wp.is_null() {
                            __wp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __w_fds.iter().zip(&__b[__rn..__rn + __wn]) {
                                    if __ws.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__ep.is_null() {
                            __ep.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __e_fds.iter().zip(&__b[__rn + __wn..]) {
                                    if __es.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        match (__tp.is_null(), __tv.as_ref()) {
                            (false, Some(__t)) => __tp.with_mut(|__dst| {
                                *__dst.tv_sec.borrow_mut() = __t.tv_sec() as i64;
                                *__dst.tv_usec.borrow_mut() = __t.tv_usec() as i64;
                            }),
                            _ => {}
                        }
                        __n
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            })
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((!(if (rset.as_pointer()).with(|__s| __s.isset((*fds.borrow())[(0) as usize])) {
            1
        } else {
            0
        } != 0) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(1) as usize], |__fd| {
            Ptr::from_string_literal(b"x")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(1_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 1_isize) as i32)
            != 0)
    );
    (rset.as_pointer()).with_mut(|__s| __s.zero());
    (rset.as_pointer()).with_mut(|__s| __s.set((*fds.borrow())[(0) as usize]));
    (*(*tv.borrow()).tv_sec.borrow_mut()) = 1_i64;
    (*(*tv.borrow()).tv_usec.borrow_mut()) = 0_i64;
    assert!(
        ((({
            let __rp = (rset.as_pointer()).clone();
            let __wp = Ptr::<CFdSet>::null().clone();
            let __ep = Ptr::<CFdSet>::null().clone();
            let __tp = (tv.as_pointer()).clone();
            let __r_fds: Vec<i32> = match __rp.is_null() {
                true => Vec::new(),
                false => __rp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __w_fds: Vec<i32> = match __wp.is_null() {
                true => Vec::new(),
                false => __wp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __e_fds: Vec<i32> = match __ep.is_null() {
                true => Vec::new(),
                false => __ep.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let mut __wanted = Vec::new();
            __wanted.extend(&__r_fds);
            __wanted.extend(&__w_fds);
            __wanted.extend(&__e_fds);
            FdRegistry::with_fds(&__wanted, |__b| {
                let __rn = __r_fds.len();
                let __wn = __w_fds.len();
                let mut __rs = nix::sys::select::FdSet::new();
                let mut __ws = nix::sys::select::FdSet::new();
                let mut __es = nix::sys::select::FdSet::new();
                for __bfd in &__b[..__rn] {
                    __rs.insert(*__bfd);
                }
                for __bfd in &__b[__rn..__rn + __wn] {
                    __ws.insert(*__bfd);
                }
                for __bfd in &__b[__rn + __wn..] {
                    __es.insert(*__bfd);
                }
                let mut __tv = match __tp.is_null() {
                    true => None,
                    false => Some(__tp.with(|__t| {
                        nix::sys::time::TimeVal::new(
                            *__t.tv_sec.borrow() as _,
                            *__t.tv_usec.borrow() as _,
                        )
                    })),
                };
                match nix::sys::select::select(
                    ((*fds.borrow())[(0) as usize] + 1),
                    match __rp.is_null() {
                        true => None,
                        false => Some(&mut __rs),
                    },
                    match __wp.is_null() {
                        true => None,
                        false => Some(&mut __ws),
                    },
                    match __ep.is_null() {
                        true => None,
                        false => Some(&mut __es),
                    },
                    __tv.as_mut(),
                ) {
                    Ok(__n) => {
                        if !__rp.is_null() {
                            __rp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __r_fds.iter().zip(&__b[..__rn]) {
                                    if __rs.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__wp.is_null() {
                            __wp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __w_fds.iter().zip(&__b[__rn..__rn + __wn]) {
                                    if __ws.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__ep.is_null() {
                            __ep.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __e_fds.iter().zip(&__b[__rn + __wn..]) {
                                    if __es.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        match (__tp.is_null(), __tv.as_ref()) {
                            (false, Some(__t)) => __tp.with_mut(|__dst| {
                                *__dst.tv_sec.borrow_mut() = __t.tv_sec() as i64;
                                *__dst.tv_usec.borrow_mut() = __t.tv_usec() as i64;
                            }),
                            _ => {}
                        }
                        __n
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            })
        } == 1) as i32)
            != 0)
    );
    assert!(
        (if (rset.as_pointer()).with(|__s| __s.isset((*fds.borrow())[(0) as usize])) {
            1
        } else {
            0
        } != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
}
pub fn test_poll_11() {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(1) as usize], |__fd| {
            Ptr::from_string_literal(b"x")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(1_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 1_isize) as i32)
            != 0)
    );
    let pfd: Value<Box<[libcc2rs::Pollfd]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| Default::default())
            .collect::<Box<[libcc2rs::Pollfd]>>(),
    ));
    (*(*pfd.borrow())[(0) as usize].fd.borrow_mut()) = (*fds.borrow())[(0) as usize];
    (*(*pfd.borrow())[(0) as usize].events.borrow_mut()) = 1_i16;
    (*(*pfd.borrow())[(0) as usize].revents.borrow_mut()) = 0_i16;
    (*(*pfd.borrow())[(1) as usize].fd.borrow_mut()) = -1_i32;
    (*(*pfd.borrow())[(1) as usize].events.borrow_mut()) = 1_i16;
    (*(*pfd.borrow())[(1) as usize].revents.borrow_mut()) = 42_i16;
    assert!(
        ((({
            let __p = (pfd.as_pointer() as Ptr<libcc2rs::Pollfd>).clone();
            let __timeout = match nix::poll::PollTimeout::try_from(0) {
                Ok(__t) => __t,
                Err(_) => panic!("poll: unsupported timeout {}", 0),
            };
            let mut __idx = Vec::new();
            let mut __wanted = Vec::new();
            let mut __events = Vec::new();
            for __i in 0..(2_u64 as usize) {
                let (__fd, __ev) = __p
                    .offset(__i)
                    .with(|__e| (*__e.fd.borrow(), *__e.events.borrow()));
                __p.offset(__i)
                    .with_mut(|__e| *__e.revents.borrow_mut() = 0);
                if __fd >= 0 {
                    __idx.push(__i);
                    __wanted.push(__fd);
                    __events.push(__ev);
                }
            }
            FdRegistry::with_fds(&__wanted, |__b| {
                let mut __pfds: Vec<nix::poll::PollFd> = __b
                    .iter()
                    .zip(&__events)
                    .map(|(&__fd, &__ev)| {
                        nix::poll::PollFd::new(__fd, nix::poll::PollFlags::from_bits_truncate(__ev))
                    })
                    .collect();
                match nix::poll::poll(&mut __pfds, __timeout) {
                    Ok(__count) => {
                        for (__slot, &__i) in __pfds.iter().zip(&__idx) {
                            let __rev = match __slot.revents() {
                                Some(__r) => __r.bits(),
                                None => 0,
                            };
                            __p.offset(__i)
                                .with_mut(|__e| *__e.revents.borrow_mut() = __rev);
                        }
                        __count
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            })
        } == 1) as i32)
            != 0)
    );
    assert!(
        ((((((*(*pfd.borrow())[(0) as usize].revents.borrow()) as i32) & 1) != 0) as i32) != 0)
    );
    assert!((((((*(*pfd.borrow())[(1) as usize].revents.borrow()) as i32) == 0) as i32) != 0));
    let ch: Value<u8> = <Value<u8>>::default();
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((ch.as_pointer()) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(1_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 1_isize) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
}
pub fn test_fileno_12() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_fileno.tmp",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __a0 = Ptr::from_string_literal(b"hello").to_any();
            let __a1 = 1_usize;
            let __a2 = 5_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        } == 5_usize) as i32)
            != 0)
    );
    assert!((((0 == 0) as i32) != 0));
    let fd: Value<i32> = Rc::new(RefCell::new((*fp.borrow()).with(|__f| __f.fd)));
    assert!(((((*fd.borrow()) >= 0) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*fp.borrow()).with(|__f| __f.fd);
            _lhs == (*fd.borrow())
        }) as i32)
            != 0)
    );
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| nix::sys::stat::fstat(__fd)) {
            Ok(__s) => {
                (st.as_pointer()).with_mut(|__st| *__st = Stat::from_libc(&__s));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*st.borrow()).st_size.borrow()) == 5_i64) as i32) != 0));
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
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
}
pub fn test_fdopen_13() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_fdopen.tmp",
    )));
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(
                ((::libc::O_WRONLY | ::libc::O_CREAT) | ::libc::O_TRUNC),
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
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(Ptr::alloc(CFile::new((*fd.borrow())))));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __a0 = Ptr::from_string_literal(b"hi").to_any();
            let __a1 = 1_usize;
            let __a2 = 2_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        } == 2_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    (*fd.borrow_mut()) = {
        let __mode = match &[].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (*path.borrow()).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(::libc::O_RDONLY),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!(((((*fd.borrow()) >= 0) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
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
        } == 2_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"hi").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
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
}
pub fn test_feof_ferror_14() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fd_io_eof.tmp",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __a0 = Ptr::from_string_literal(b"ab").to_any();
            let __a1 = 1_usize;
            let __a2 = 2_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        } == 2_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(((!((*fp.borrow()).with(|__f| __f.eof) as i32 != 0) as i32) != 0));
    assert!(((!((*fp.borrow()).with(|__f| __f.err) as i32 != 0) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any();
            let __a1 = 1_usize;
            let __a2 = 2_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 2_usize) as i32)
            != 0)
    );
    assert!(((!((*fp.borrow()).with(|__f| __f.eof) as i32 != 0) as i32) != 0));
    assert!(
        ((({
            let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any();
            let __a1 = 1_usize;
            let __a2 = 1_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 0_usize) as i32)
            != 0)
    );
    assert!(((*fp.borrow()).with(|__f| __f.eof) as i32 != 0));
    assert!(((!((*fp.borrow()).with(|__f| __f.err) as i32 != 0) as i32) != 0));
    assert!(
        (((match (*fp.borrow()).with_mut(|__v: &mut CFile| __v.seek(0_i64, 0)) {
            -1 => -1,
            _ => 0,
        } == 0) as i32)
            != 0)
    );
    assert!(((!((*fp.borrow()).with(|__f| __f.eof) as i32 != 0) as i32) != 0));
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
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
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_open_read_write_0() });
    ({ test_pipe_1() });
    ({ test_socket_listen_2() });
    ({ test_sockopt_3() });
    ({ test_lseek_4() });
    ({ test_ftruncate_5() });
    ({ test_fstat_6() });
    ({ test_isatty_7() });
    ({ test_tcgetattr_8() });
    ({ test_fcntl_9() });
    ({ test_select_10() });
    ({ test_poll_11() });
    ({ test_fileno_12() });
    ({ test_fdopen_13() });
    ({ test_feof_ferror_14() });
    return 0;
}
