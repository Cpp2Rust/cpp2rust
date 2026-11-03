extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_name_0(mut fd: i32, mut path: Ptr<i8>) {
    let full: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    match ({ ((full.as_pointer()) as Ptr<libcc2rs::SockaddrStorage>).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memset((165) as u8, 128usize as usize);
            (__tmp_0).clone()
        }
    };
    let actual: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        (((match (
            { (full.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() },
            { (actual.as_pointer()) },
        ) {
            (mut __tmp_1, mut __tmp_2) => {
                match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
                    Ok(__ss) => {
                        Sockaddr::encode(&__ss, &__tmp_1, &__tmp_2);
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(({ (((({ (*full.borrow()).ss_family } as i32) == 1) as i32) != 0) }));
    assert!(({ (((((*actual.borrow()) as usize) >= 2_usize) as i32) != 0) }));
    assert!(({ (((((*actual.borrow()) as usize) < 128usize) as i32) != 0) }));
    if !(path).is_null() {
        assert!(
            (((((array_field_ptr!(
                ((full.as_pointer()).reinterpret_cast::<libcc2rs::SockaddrUn>()),
                sun_path
            ) as Ptr::<i8>) as Ptr<i8>)
                .to_any()
                .memcmp(&(path).to_any(), path.to_c_string_iterator().count())
                == 0) as i32)
                != 0)
        );
    }
    let mut capacities: [libc::socklen_t; 6] = [
        (0 as libc::socklen_t),
        (1 as libc::socklen_t),
        (2 as libc::socklen_t),
        (*actual.borrow()).wrapping_sub((1 as libc::socklen_t)),
        (*actual.borrow()),
        (*actual.borrow()).wrapping_add((1 as libc::socklen_t)),
    ];
    let mut i: u32 = 0_u32;
    'loop_: while ((((i as usize)
        < (::std::mem::size_of::<[libc::socklen_t; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::socklen_t>() as usize)))
        as i32)
        != 0)
    {
        let out: Value<Box<[u8]>> =
            Rc::new(RefCell::new((0..136).map(|_| 0_u8).collect::<Box<[u8]>>()));
        match ({ ((out.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any() },) {
            (mut __tmp_3,) => {
                __tmp_3.memset((165) as u8, ::std::mem::size_of::<[u8; 136]>() as usize);
                (__tmp_3).clone()
            }
        };
        let len: Value<libc::socklen_t> = Rc::new(RefCell::new(capacities[(i) as usize]));
        assert!(
            (((match (
                { (out.as_pointer() as Ptr::<u8>).reinterpret_cast::<libcc2rs::Sockaddr>() },
                { (len.as_pointer()) },
            ) {
                (mut __tmp_4, mut __tmp_5) => {
                    match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
                        Ok(__ss) => {
                            Sockaddr::encode(&__ss, &__tmp_4, &__tmp_5);
                            0
                        }
                        Err(__e) => {
                            libcc2rs::cpp2rust_errno().write(__e as i32);
                            -1
                        }
                    }
                }
            } == 0) as i32)
                != 0)
        );
        assert!(({ ((((*len.borrow()) == (*actual.borrow())) as i32) != 0) }));
        let mut copied: u32 = (if (((capacities[(i) as usize] < (*actual.borrow())) as i32) != 0) {
            capacities[(i) as usize]
        } else {
            (*actual.borrow())
        } as u32);
        assert!(
            (((((out.as_pointer() as Ptr::<u8>) as Ptr<u8>)
                .to_any()
                .memcmp(
                    &((full.as_pointer()) as Ptr<libcc2rs::SockaddrStorage>).to_any(),
                    (copied as usize)
                )
                == 0) as i32)
                != 0)
        );
        let mut j: u32 = copied;
        'loop_: while ((((j as usize) < ::std::mem::size_of::<[u8; 136]>()) as i32) != 0) {
            assert!(({ (((((*out.borrow())[(j) as usize] as i32) == 165) as i32) != 0) }));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fd: i32 = {
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
    assert!((((fd >= 0) as i32) != 0));
    ({ check_name_0(fd, Ptr::<i8>::null()) });
    let addr: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    match ({ ((addr.as_pointer()) as Ptr<libcc2rs::SockaddrStorage>).to_any() },) {
        (mut __tmp_6,) => {
            __tmp_6.memset((0) as u8, 128usize as usize);
            (__tmp_6).clone()
        }
    };
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        (((match (
            { (addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() },
            { (len.as_pointer()) },
        ) {
            (mut __tmp_7, mut __tmp_8) => {
                match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
                    Ok(__ss) => {
                        Sockaddr::encode(&__ss, &__tmp_7, &__tmp_8);
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            }
        } == 0) as i32)
            != 0)
    );
    let mut un: Ptr<libcc2rs::SockaddrUn> =
        (addr.as_pointer()).reinterpret_cast::<libcc2rs::SockaddrUn>();
    match ({ ((array_field_ptr!(un, sun_path) as Ptr<i8>) as Ptr<i8>).to_any() },) {
        (mut __tmp_9,) => {
            __tmp_9.memcpy(
                &Ptr::<i8>::from_string_literal(b"socket-unix-name").to_any(),
                ::std::mem::size_of::<[i8; 17]>() as usize,
            );
            (__tmp_9).clone()
        }
    };
    assert!(
        (((match Sockaddr::decode(
            &(addr.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
            (((2_usize as usize).wrapping_add(
                (array_field_ptr!(un, sun_path) as Ptr::<i8>)
                    .to_c_string_iterator()
                    .count()
            )) as libc::socklen_t)
        ) {
            Some(__addr) => match nix::sys::socket::bind(fd, &*__addr) {
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
        } == 0) as i32)
            != 0)
    );
    ({ check_name_0(fd, (array_field_ptr!(un, sun_path) as Ptr<i8>)) });
    assert!((((FdRegistry::close(fd) == 0) as i32) != 0));
    assert!(
        (((match nix::unistd::unlink(
            (array_field_ptr!(un, sun_path) as Ptr::<i8>)
                .to_rust_string()
                .as_str()
        ) {
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
pub fn __cpp2rust_init_globals() {}
