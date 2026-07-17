extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_ipv4_literal_0() {
    let mut hints: ::libc::addrinfo = ::libc::addrinfo {
        ai_flags: 0,
        ai_family: 0_i32,
        ai_socktype: 0_i32,
        ai_protocol: 0_i32,
        ai_addrlen: 0_u32,
        ai_addr: std::ptr::null_mut(),
        ai_canonname: std::ptr::null_mut(),
        ai_next: std::ptr::null_mut(),
    };
    hints.ai_family = 2;
    hints.ai_socktype = libc::SOCK_STREAM;
    let mut res: *mut ::libc::addrinfo = std::ptr::null_mut();
    assert!(
        ((((libc::getaddrinfo(
            (c"127.0.0.1".as_ptr().cast_mut()).cast_const(),
            (c"8080".as_ptr().cast_mut()).cast_const(),
            (&mut hints as *mut ::libc::addrinfo).cast_const(),
            (&mut res as *mut *mut ::libc::addrinfo)
        )) == (0)) as i32)
            != 0)
    );
    assert!((((!((res).is_null())) as i32) != 0));
    assert!((((((*res).ai_family) == (2)) as i32) != 0));
    assert!((((((*res).ai_socktype) == (libc::SOCK_STREAM)) as i32) != 0));
    assert!(
        (((((*res).ai_addrlen as usize) == (::std::mem::size_of::<::libc::sockaddr_in>())) as i32)
            != 0)
    );
    assert!((((!(((*res).ai_addr).is_null())) as i32) != 0));
    let mut sin: *mut ::libc::sockaddr_in = ((*res).ai_addr as *mut ::libc::sockaddr_in);
    assert!((((((*sin).sin_family as i32) == (2)) as i32) != 0));
    let mut port_be: [u8; 2] = [(((8080) / (256)) as u8), (((8080) % (256)) as u8)];
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((&mut (*sin).sin_port as *mut u16) as *const u16 as *const ::libc::c_void)
                    as *const u8,
                2_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (port_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                2_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    let mut addr_be: [u8; 4] = [127_u8, 0_u8, 0_u8, 1_u8];
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((&mut (*sin).sin_addr as *mut ::libc::in_addr) as *const ::libc::in_addr
                    as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (addr_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    libc::freeaddrinfo(res);
}
pub unsafe fn test_ipv6_literal_1() {
    let mut hints: ::libc::addrinfo = ::libc::addrinfo {
        ai_flags: 0,
        ai_family: 0_i32,
        ai_socktype: 0_i32,
        ai_protocol: 0_i32,
        ai_addrlen: 0_u32,
        ai_addr: std::ptr::null_mut(),
        ai_canonname: std::ptr::null_mut(),
        ai_next: std::ptr::null_mut(),
    };
    hints.ai_family = 10;
    hints.ai_socktype = libc::SOCK_STREAM;
    let mut res: *mut ::libc::addrinfo = std::ptr::null_mut();
    assert!(
        ((((libc::getaddrinfo(
            (c"::1".as_ptr().cast_mut()).cast_const(),
            (c"443".as_ptr().cast_mut()).cast_const(),
            (&mut hints as *mut ::libc::addrinfo).cast_const(),
            (&mut res as *mut *mut ::libc::addrinfo)
        )) == (0)) as i32)
            != 0)
    );
    assert!((((!((res).is_null())) as i32) != 0));
    assert!((((((*res).ai_family) == (10)) as i32) != 0));
    assert!(
        (((((*res).ai_addrlen as usize) == (::std::mem::size_of::<::libc::sockaddr_in6>()))
            as i32)
            != 0)
    );
    assert!((((!(((*res).ai_addr).is_null())) as i32) != 0));
    let mut sin6: *mut ::libc::sockaddr_in6 = ((*res).ai_addr as *mut ::libc::sockaddr_in6);
    assert!((((((*sin6).sin6_family as i32) == (10)) as i32) != 0));
    let mut port_be: [u8; 2] = [(((443) / (256)) as u8), (((443) % (256)) as u8)];
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((&mut (*sin6).sin6_port as *mut u16) as *const u16 as *const ::libc::c_void)
                    as *const u8,
                2_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (port_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                2_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    let mut addr_be: [u8; 16] = [
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        1_u8,
    ];
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((&mut (*sin6).sin6_addr as *mut ::libc::in6_addr) as *const ::libc::in6_addr
                    as *const ::libc::c_void) as *const u8,
                16_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (addr_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                16_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    libc::freeaddrinfo(res);
}
pub unsafe fn test_family_mismatch_2() {
    let mut hints: ::libc::addrinfo = ::libc::addrinfo {
        ai_flags: 0,
        ai_family: 0_i32,
        ai_socktype: 0_i32,
        ai_protocol: 0_i32,
        ai_addrlen: 0_u32,
        ai_addr: std::ptr::null_mut(),
        ai_canonname: std::ptr::null_mut(),
        ai_next: std::ptr::null_mut(),
    };
    hints.ai_family = 10;
    hints.ai_socktype = libc::SOCK_STREAM;
    let mut res: *mut ::libc::addrinfo = std::ptr::null_mut();
    assert!(
        ((((libc::getaddrinfo(
            (c"127.0.0.1".as_ptr().cast_mut()).cast_const(),
            (c"80".as_ptr().cast_mut()).cast_const(),
            (&mut hints as *mut ::libc::addrinfo).cast_const(),
            (&mut res as *mut *mut ::libc::addrinfo)
        )) != (0)) as i32)
            != 0)
    );
}
pub unsafe fn test_null_hints_3() {
    let mut res: *mut ::libc::addrinfo = std::ptr::null_mut();
    assert!(
        ((((libc::getaddrinfo(
            (c"127.0.0.1".as_ptr().cast_mut()).cast_const(),
            (c"80".as_ptr().cast_mut()).cast_const(),
            std::ptr::null(),
            (&mut res as *mut *mut ::libc::addrinfo)
        )) == (0)) as i32)
            != 0)
    );
    assert!((((!((res).is_null())) as i32) != 0));
    assert!((((((*res).ai_family) == (2)) as i32) != 0));
    let mut sin: *mut ::libc::sockaddr_in = ((*res).ai_addr as *mut ::libc::sockaddr_in);
    let mut addr_be: [u8; 4] = [127_u8, 0_u8, 0_u8, 1_u8];
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((&mut (*sin).sin_addr as *mut ::libc::in_addr) as *const ::libc::in_addr
                    as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (addr_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    libc::freeaddrinfo(res);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_ipv4_literal_0() });
    (unsafe { test_ipv6_literal_1() });
    (unsafe { test_family_mismatch_2() });
    (unsafe { test_null_hints_3() });
    return 0;
}
