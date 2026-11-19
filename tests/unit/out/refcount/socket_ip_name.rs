extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_buf_0(
    mut fd: i32,
    mut buf: Ptr<u8>,
    mut size: u32,
    mut cap: libc::socklen_t,
    mut full: AnyPtr,
    mut actual: libc::socklen_t,
) {
    match ({ (buf).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memset((165) as u8, (size as usize));
            (__tmp_0).clone()
        }
    };
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new(cap));
    assert!(
        (match ({ buf.reinterpret_cast::<libcc2rs::Sockaddr>() }, {
            (len.as_pointer())
        },)
        {
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
        } == 0)
    );
    let mut copied: u32 = (if (cap < actual) { cap } else { actual } as u32);
    assert!(((buf).to_any().memcmp(&full, (copied as usize)) == 0));
    {
        let mut j: u32 = copied;
        'loop_: while (j < size) {
            assert!((((elem!(buf, j).read()) as i32) == 165));
            j.prefix_inc();
        }
    }
}
pub fn check_name_1(mut family: i32) {
    let mut fd: i32 = {
        let __family = match family {
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
    assert!((fd >= 0));
    let full: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    match ({ (full.as_pointer()).to_any() },) {
        (mut __tmp_3,) => {
            __tmp_3.memset((165) as u8, 128usize);
            (__tmp_3).clone()
        }
    };
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        (match (
            { (full.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() },
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
        } == 0)
    );
    assert!(({ (({ (*full.borrow()).ss_family } as i32) == family) }));
    let mut actual: libc::socklen_t = (if (family == libc::AF_INET) {
        16usize
    } else {
        28usize
    } as libc::socklen_t);
    let mut capacities: [libc::socklen_t; 6] = [
        (0 as libc::socklen_t),
        (1 as libc::socklen_t),
        (2 as libc::socklen_t),
        (actual).wrapping_sub((1 as libc::socklen_t)),
        actual,
        (actual).wrapping_add((1 as libc::socklen_t)),
    ];
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize)
            < (::std::mem::size_of::<[libc::socklen_t; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<libc::socklen_t>() as usize)))
        {
            let out: Value<Box<[u8]>> =
                Rc::new(RefCell::new((0..136).map(|_| 0_u8).collect::<Box<[u8]>>()));
            ({
                let _buf: Ptr<u8> = out.decay();
                let _size: u32 = (::std::mem::size_of::<[u8; 136]>() as u32);
                check_buf_0(
                    fd,
                    _buf,
                    _size,
                    capacities[(i) as usize],
                    (full.as_pointer()).to_any(),
                    actual,
                )
            });
            let mut size: u32 = (if (capacities[(i) as usize] != 0) {
                capacities[(i) as usize]
            } else {
                (1 as libc::socklen_t)
            } as u32);
            let mut small: Ptr<u8> =
                libcc2rs::malloc_refcount((size as usize)).reinterpret_cast::<u8>();
            assert!(!(small).is_null());
            ({
                check_buf_0(
                    fd,
                    (small).clone(),
                    size,
                    capacities[(i) as usize],
                    (full.as_pointer()).to_any(),
                    actual,
                )
            });
            libcc2rs::free_refcount((small).to_any());
            i.prefix_inc();
        }
    }
    assert!((FdRegistry::close(fd) == 0));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ check_name_1(libc::AF_INET) });
    ({ check_name_1(libc::AF_INET6) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
