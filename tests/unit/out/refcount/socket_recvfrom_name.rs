extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_from_0(
    mut receiver: i32,
    mut sender: i32,
    mut expected: Ptr<libcc2rs::SockaddrStorage>,
    mut expected_len: libc::socklen_t,
) {
    let sent: Value<i8> = Rc::new(RefCell::new((('x' as i32) as i8)));
    let received: Value<i8> = Rc::new(RefCell::new(0_i8));
    assert!(
        ({
            let __buf = (sent.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    let from: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    {
        let mut __tmp_0 = { (from.as_pointer()).to_any() };
        __tmp_0.memset((165) as u8, 128usize);
        (__tmp_0).clone()
    };
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_1 = { (from.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_2 = { (len.as_pointer()) };
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !__tmp_1.is_null() {
                                Sockaddr::encode(&__ss, &__tmp_1, &__tmp_2)
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ ((*len.borrow()) == expected_len) }));
    assert!(({ (((*received.borrow()) as i32) == ((*sent.borrow()) as i32)) }));
    assert!(
        ((from.as_pointer())
            .to_any()
            .memcmp(&(expected).to_any(), ({ ((*len.borrow()) as usize) }))
            == 0)
    );
    {
        let mut i: u32 = ((*len.borrow()) as u32);
        'loop_: while ((i as usize) < 128usize) {
            assert!(
                (((elem!(((from.as_pointer()).reinterpret_cast::<u8>()), i).read()) as i32) == 165)
            );
            i.prefix_inc();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut receiver: i32 = {
        let __family = match 1 {
            ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
            ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
            ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
            __d => panic!("socket: unsupported domain {__d}"),
        };
        let __flags = nix::sys::socket::SockFlag::from_bits_truncate(libc::SOCK_DGRAM);
        let __ty = match libc::SOCK_DGRAM & !nix::sys::socket::SockFlag::all().bits() {
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
    };
    let mut sender: i32 = {
        let __family = match 1 {
            ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
            ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
            ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
            __d => panic!("socket: unsupported domain {__d}"),
        };
        let __flags = nix::sys::socket::SockFlag::from_bits_truncate(libc::SOCK_DGRAM);
        let __ty = match libc::SOCK_DGRAM & !nix::sys::socket::SockFlag::all().bits() {
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
    };
    assert!((receiver >= 0));
    assert!((sender >= 0));
    let addr: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_3 = { (addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_4 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(receiver) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_3, &__tmp_4);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    let mut un: Ptr<libcc2rs::SockaddrUn> =
        (addr.as_pointer()).reinterpret_cast::<libcc2rs::SockaddrUn>();
    {
        let mut __tmp_5 = { (array_field_ptr!(un, sun_path)).to_any() };
        __tmp_5.memcpy(
            &(Ptr::<i8>::from_string_literal(b"recvfrom-receiver")).to_any(),
            ::std::mem::size_of::<[i8; 18]>(),
        );
        (__tmp_5).clone()
    };
    (*len.borrow_mut()) = ((((2_usize as usize).wrapping_add(
        array_field_ptr!(un, sun_path)
            .to_c_string_iterator()
            .count(),
    ))
    .wrapping_add(1_usize)) as libc::socklen_t);
    assert!(
        (match Sockaddr::decode(
            &(addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
            ({ (*len.borrow()) })
        ) {
            Some(__addr) => match nix::sys::socket::bind(receiver, &*__addr) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            },
            None => {
                libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                -1
            }
        } == 0)
    );
    assert!(
        (match Sockaddr::decode(
            &(addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
            ({ (*len.borrow()) })
        ) {
            Some(__addr) => match nix::sys::socket::connect(sender, &*__addr) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            },
            None => {
                libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                -1
            }
        } == 0)
    );
    let received: Value<i8> = Rc::new(RefCell::new(0_i8));
    assert!(
        ({
            let __buf = (Ptr::<i8>::from_string_literal(b"x"))
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    let from: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    let from_len: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_6 = { (from.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_7 = { (from_len.as_pointer()) };
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !__tmp_6.is_null() {
                                Sockaddr::encode(&__ss, &__tmp_6, &__tmp_7)
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ (((*received.borrow()) as i32) == ('x' as i32)) }));
    assert!(
        ({
            let __buf = (Ptr::<i8>::from_string_literal(b"x"))
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(
        ({
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !Ptr::<libcc2rs::Sockaddr>::null().is_null() {
                                Sockaddr::encode(
                                    &__ss,
                                    &Ptr::<libcc2rs::Sockaddr>::null(),
                                    &Ptr::<libc::socklen_t>::null(),
                                )
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ (((*received.borrow()) as i32) == ('x' as i32)) }));
    (*received.borrow_mut()) = 0_i8;
    (*len.borrow_mut()) = (77 as libc::socklen_t);
    assert!(
        ({
            let __buf = (Ptr::<i8>::from_string_literal(b"x"))
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(
        ({
            let mut __tmp_8 = { (len.as_pointer()) };
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !Ptr::<libcc2rs::Sockaddr>::null().is_null() {
                                Sockaddr::encode(
                                    &__ss,
                                    &Ptr::<libcc2rs::Sockaddr>::null(),
                                    &__tmp_8,
                                )
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ ((*len.borrow()) == (77 as libc::socklen_t)) }));
    assert!(({ (((*received.borrow()) as i32) == ('x' as i32)) }));
    {
        let mut __tmp_9 = { (array_field_ptr!(un, sun_path)).to_any() };
        __tmp_9.memcpy(
            &(Ptr::<i8>::from_string_literal(b"recvfrom-sender")).to_any(),
            ::std::mem::size_of::<[i8; 16]>(),
        );
        (__tmp_9).clone()
    };
    (*len.borrow_mut()) = ((((2_usize as usize).wrapping_add(
        array_field_ptr!(un, sun_path)
            .to_c_string_iterator()
            .count(),
    ))
    .wrapping_add(1_usize)) as libc::socklen_t);
    assert!(
        (match Sockaddr::decode(
            &(addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
            ({ (*len.borrow()) })
        ) {
            Some(__addr) => match nix::sys::socket::bind(sender, &*__addr) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            },
            None => {
                libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                -1
            }
        } == 0)
    );
    (*len.borrow_mut()) = (128usize as libc::socklen_t);
    assert!(
        ({
            let mut __tmp_10 = { (addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_11 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(sender) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_10, &__tmp_11);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    ({ check_from_0(receiver, sender, (addr.as_pointer()), (*len.borrow())) });
    (*received.borrow_mut()) = 0_i8;
    assert!(
        ({
            let __buf = (Ptr::<i8>::from_string_literal(b"x"))
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(
        ({
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !Ptr::<libcc2rs::Sockaddr>::null().is_null() {
                                Sockaddr::encode(
                                    &__ss,
                                    &Ptr::<libcc2rs::Sockaddr>::null(),
                                    &Ptr::<libc::socklen_t>::null(),
                                )
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ (((*received.borrow()) as i32) == ('x' as i32)) }));
    (*received.borrow_mut()) = 0_i8;
    (*len.borrow_mut()) = (77 as libc::socklen_t);
    assert!(
        ({
            let __buf = (Ptr::<i8>::from_string_literal(b"x"))
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(
        ({
            let mut __tmp_12 = { (len.as_pointer()) };
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !Ptr::<libcc2rs::Sockaddr>::null().is_null() {
                                Sockaddr::encode(
                                    &__ss,
                                    &Ptr::<libcc2rs::Sockaddr>::null(),
                                    &__tmp_12,
                                )
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ ((*len.borrow()) == (77 as libc::socklen_t)) }));
    assert!(({ (((*received.borrow()) as i32) == ('x' as i32)) }));
    assert!((FdRegistry::close(sender) == 0));
    assert!((FdRegistry::close(receiver) == 0));
    assert!(
        (match Ptr::<i8>::from_string_literal(b"recvfrom-sender")
            .with_c_bytes(|__path| nix::unistd::unlink(__path))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0)
    );
    assert!(
        (match Ptr::<i8>::from_string_literal(b"recvfrom-receiver")
            .with_c_bytes(|__path| nix::unistd::unlink(__path))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
