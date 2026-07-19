// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f5(a0: i32, a1: Ptr<u8>, a2: AnyPtr) -> i32 {
    if a0 == libc::AF_INET {
        match a1.to_rust_string().parse::<std::net::Ipv4Addr>() {
            Ok(__ip) => {
                a2.reinterpret_cast::<u8>()
                    .with_slice_mut(4, |__s| __s.copy_from_slice(&__ip.octets()));
                1
            }
            Err(_) => 0,
        }
    } else if a0 == libc::AF_INET6 {
        match a1.to_rust_string().parse::<std::net::Ipv6Addr>() {
            Ok(__ip) => {
                a2.reinterpret_cast::<u8>()
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

fn f6(a0: i32, a1: AnyPtr, a2: Ptr<u8>, a3: u32) -> Ptr<u8> {
    let __text = if a0 == libc::AF_INET {
        let __b: [u8; 4] = a1
            .reinterpret_cast::<u8>()
            .with_slice(4, |__s| __s.try_into().unwrap());
        Some(std::net::Ipv4Addr::from(__b).to_string())
    } else if a0 == libc::AF_INET6 {
        let __b: [u8; 16] = a1
            .reinterpret_cast::<u8>()
            .with_slice(16, |__s| __s.try_into().unwrap());
        Some(std::net::Ipv6Addr::from(__b).to_string())
    } else {
        None
    };
    match __text {
        Some(__s) if (__s.len() as u32) < a3 => {
            let __n = __s.len();
            a2.with_slice_mut(__n + 1, |__sl| {
                __sl[..__n].copy_from_slice(__s.as_bytes());
                __sl[__n] = 0;
            });
            a2.clone()
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
