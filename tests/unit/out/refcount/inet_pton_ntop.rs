extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_s6_addr_access_0() {
    let loopback: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        1_u8,
    ])));
    let addr: Value<libcc2rs::In6Addr> = Rc::new(RefCell::new(Default::default()));
    let mut ap: Ptr<libcc2rs::In6Addr> = (addr.as_pointer());
    match ({ ((In6Addr::s6_addr(addr.as_pointer()) as Ptr<u8>) as Ptr<u8>).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(
                &((loopback.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                16_usize as usize,
            );
            (__tmp_0).clone()
        }
    };
    assert!(
        (((((In6Addr::s6_addr((ap).clone()) as Ptr::<u8>) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((loopback.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any(),
                16_usize
            )
            == 0) as i32)
            != 0)
    );
    elem!((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>), 0).write(32_u8);
    elem!((In6Addr::s6_addr((ap).clone()) as Ptr::<u8>), 1).write(1_u8);
    assert!(
        ((((((((elem!((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>), 0).read()) as i32) == 32)
            as i32)
            != 0)
            && (((((elem!((In6Addr::s6_addr((ap).clone()) as Ptr::<u8>), 1).read()) as i32) == 1)
                as i32)
                != 0)) as i32)
            != 0)
    );
    elem!((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>), 15).write({
        (((elem!((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>), 15).read()) as i32) + 4) as u8
    });
    elem!((In6Addr::s6_addr((ap).clone()) as Ptr::<u8>), 15).with_mut(|__v| __v.postfix_inc());
    assert!(
        (((((elem!((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>), 15).read()) as i32) == 6)
            as i32)
            != 0)
    );
    let mut bytes: Ptr<u8> = (In6Addr::s6_addr(addr.as_pointer()) as Ptr<u8>);
    elem!(bytes, 2).write(7_u8);
    assert!(
        (((((elem!((In6Addr::s6_addr((ap).clone()) as Ptr::<u8>), 2).read()) as i32) == 7) as i32)
            != 0)
    );
    let mut fourth: Ptr<u8> = ((In6Addr::s6_addr((ap).clone()) as Ptr<u8>).offset((3) as isize));
    fourth.write(8_u8);
    assert!(
        (((((elem!((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>), 3).read()) as i32) == 8)
            as i32)
            != 0)
    );
    match ({ ((In6Addr::s6_addr((ap).clone()) as Ptr<u8>) as Ptr<u8>).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(
                &((loopback.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                16_usize as usize,
            );
            (__tmp_1).clone()
        }
    };
    assert!(
        (((((In6Addr::s6_addr(addr.as_pointer()) as Ptr::<u8>) as Ptr<u8>)
            .to_any()
            .memcmp(
                &((loopback.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any(),
                16_usize
            )
            == 0) as i32)
            != 0)
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_s6_addr_access_0() });
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..16).map(|_| 0_u8).collect::<Box<[u8]>>()));
    assert!(
        (((match ({ ((buf.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },) {
            (mut __tmp_2,) => {
                if libc::AF_INET == libc::AF_INET {
                    match Ptr::<i8>::from_string_literal(b"1.2.3.4")
                        .to_rust_string()
                        .parse::<std::net::Ipv4Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_2
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(4, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else if libc::AF_INET == libc::AF_INET6 {
                    match Ptr::<i8>::from_string_literal(b"1.2.3.4")
                        .to_rust_string()
                        .parse::<std::net::Ipv6Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_2
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(16, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else {
                    libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                    -1
                }
            }
        } == 1) as i32)
            != 0)
    );
    assert!(
        ({
            ((((((((((((((*buf.borrow())[(0) as usize] as i32) == 1) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == 2) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(2) as usize] as i32) == 3) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(3) as usize] as i32) == 4) as i32) != 0))
                as i32)
                != 0)
        })
    );
    assert!(
        (((match ({ ((buf.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },) {
            (mut __tmp_3,) => {
                if libc::AF_INET == libc::AF_INET {
                    match Ptr::<i8>::from_string_literal(b"999.1.1.1")
                        .to_rust_string()
                        .parse::<std::net::Ipv4Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_3
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(4, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else if libc::AF_INET == libc::AF_INET6 {
                    match Ptr::<i8>::from_string_literal(b"999.1.1.1")
                        .to_rust_string()
                        .parse::<std::net::Ipv6Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_3
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(16, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else {
                    libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match ({ ((buf.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },) {
            (mut __tmp_4,) => {
                if libc::AF_INET == libc::AF_INET {
                    match Ptr::<i8>::from_string_literal(b"not an ip")
                        .to_rust_string()
                        .parse::<std::net::Ipv4Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_4
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(4, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else if libc::AF_INET == libc::AF_INET6 {
                    match Ptr::<i8>::from_string_literal(b"not an ip")
                        .to_rust_string()
                        .parse::<std::net::Ipv6Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_4
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(16, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else {
                    libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match ({ ((buf.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },) {
            (mut __tmp_5,) => {
                if libc::AF_INET6 == libc::AF_INET {
                    match Ptr::<i8>::from_string_literal(b"::1")
                        .to_rust_string()
                        .parse::<std::net::Ipv4Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_5
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(4, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else if libc::AF_INET6 == libc::AF_INET6 {
                    match Ptr::<i8>::from_string_literal(b"::1")
                        .to_rust_string()
                        .parse::<std::net::Ipv6Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_5
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(16, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else {
                    libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                    -1
                }
            }
        } == 1) as i32)
            != 0)
    );
    assert!(
        ({
            ((((((((*buf.borrow())[(0) as usize] as i32) == 0) as i32) != 0)
                && (((((*buf.borrow())[(15) as usize] as i32) == 1) as i32) != 0))
                as i32)
                != 0)
        })
    );
    assert!(
        (((match ({ ((buf.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },) {
            (mut __tmp_6,) => {
                if libc::AF_INET6 == libc::AF_INET {
                    match Ptr::<i8>::from_string_literal(b"2001:db8::5")
                        .to_rust_string()
                        .parse::<std::net::Ipv4Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_6
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(4, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else if libc::AF_INET6 == libc::AF_INET6 {
                    match Ptr::<i8>::from_string_literal(b"2001:db8::5")
                        .to_rust_string()
                        .parse::<std::net::Ipv6Addr>()
                    {
                        Ok(__ip) => {
                            __tmp_6
                                .reinterpret_cast::<u8>()
                                .with_slice_mut(16, |__s| __s.copy_from_slice(&__ip.octets()));
                            1
                        }
                        Err(_) => 0,
                    }
                } else {
                    libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                    -1
                }
            }
        } == 1) as i32)
            != 0)
    );
    assert!(
        ({
            (((((((((((*buf.borrow())[(0) as usize] as i32) == 32) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == 1) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(15) as usize] as i32) == 5) as i32) != 0))
                as i32)
                != 0)
        })
    );
    let text: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let four: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([10_u8, 0_u8, 0_u8, 1_u8])));
    assert!(
        ((({
            let mut __it1 = match ({ ((four.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any() }, {
                (text.as_pointer() as Ptr<i8>)
            }) {
                (mut __tmp_7, mut __tmp_8) => {
                    let __text = if libc::AF_INET == libc::AF_INET {
                        let __b: [u8; 4] = __tmp_7
                            .reinterpret_cast::<u8>()
                            .with_slice(4, |__s| __s.try_into().unwrap());
                        Some(std::net::Ipv4Addr::from(__b).to_string())
                    } else if libc::AF_INET == libc::AF_INET6 {
                        let __b: [u8; 16] = __tmp_7
                            .reinterpret_cast::<u8>()
                            .with_slice(16, |__s| __s.try_into().unwrap());
                        Some(std::net::Ipv6Addr::from(__b).to_string())
                    } else {
                        None
                    };
                    match __text {
                        Some(__s)
                            if (__s.len() as u32)
                                < (::std::mem::size_of::<[i8; 64]>() as libc::socklen_t) =>
                        {
                            let __n = __s.len();
                            __tmp_8.write_c_bytes(__s.as_bytes());
                            elem!(__tmp_8, __n).write(0);
                            (__tmp_8).clone()
                        }
                        Some(_) => {
                            libcc2rs::cpp2rust_errno().write(::libc::ENOSPC);
                            Ptr::null()
                        }
                        None => {
                            libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                            Ptr::null()
                        }
                    }
                }
            }
            .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"10.0.0.1").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    let sixteen: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ])));
    (*sixteen.borrow_mut())[(15) as usize] = 1_u8;
    assert!(
        ((({
            let mut __it1 = match (
                { ((sixteen.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any() },
                { (text.as_pointer() as Ptr<i8>) },
            ) {
                (mut __tmp_9, mut __tmp_10) => {
                    let __text = if libc::AF_INET6 == libc::AF_INET {
                        let __b: [u8; 4] = __tmp_9
                            .reinterpret_cast::<u8>()
                            .with_slice(4, |__s| __s.try_into().unwrap());
                        Some(std::net::Ipv4Addr::from(__b).to_string())
                    } else if libc::AF_INET6 == libc::AF_INET6 {
                        let __b: [u8; 16] = __tmp_9
                            .reinterpret_cast::<u8>()
                            .with_slice(16, |__s| __s.try_into().unwrap());
                        Some(std::net::Ipv6Addr::from(__b).to_string())
                    } else {
                        None
                    };
                    match __text {
                        Some(__s)
                            if (__s.len() as u32)
                                < (::std::mem::size_of::<[i8; 64]>() as libc::socklen_t) =>
                        {
                            let __n = __s.len();
                            __tmp_10.write_c_bytes(__s.as_bytes());
                            elem!(__tmp_10, __n).write(0);
                            (__tmp_10).clone()
                        }
                        Some(_) => {
                            libcc2rs::cpp2rust_errno().write(::libc::ENOSPC);
                            Ptr::null()
                        }
                        None => {
                            libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                            Ptr::null()
                        }
                    }
                }
            }
            .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"::1").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((((match (
            { ((four.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },
            { (text.as_pointer() as Ptr::<i8>) },
        ) {
            (mut __tmp_11, mut __tmp_12) => {
                {
                    let __text = if libc::AF_INET == libc::AF_INET {
                        let __b: [u8; 4] = __tmp_11
                            .reinterpret_cast::<u8>()
                            .with_slice(4, |__s| __s.try_into().unwrap());
                        Some(std::net::Ipv4Addr::from(__b).to_string())
                    } else if libc::AF_INET == libc::AF_INET6 {
                        let __b: [u8; 16] = __tmp_11
                            .reinterpret_cast::<u8>()
                            .with_slice(16, |__s| __s.try_into().unwrap());
                        Some(std::net::Ipv6Addr::from(__b).to_string())
                    } else {
                        None
                    };
                    match __text {
                        Some(__s) if (__s.len() as u32) < (4 as libc::socklen_t) => {
                            let __n = __s.len();
                            __tmp_12.write_c_bytes(__s.as_bytes());
                            elem!(__tmp_12, __n).write(0);
                            (__tmp_12).clone()
                        }
                        Some(_) => {
                            libcc2rs::cpp2rust_errno().write(::libc::ENOSPC);
                            Ptr::null()
                        }
                        None => {
                            libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
                            Ptr::null()
                        }
                    }
                }
            }
        })
        .is_null()) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
