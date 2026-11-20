extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_name_0(mut addr: *mut libc::sockaddr, mut expected: libc::socklen_t) {
    let mut receiver: i32 = libc::socket(((*addr).sa_family as i32), libc::SOCK_DGRAM, 0);
    let mut sender: i32 = libc::socket(((*addr).sa_family as i32), libc::SOCK_DGRAM, 0);
    assert!(((receiver) >= (0)));
    assert!(((sender) >= (0)));
    let mut local: libc::sockaddr_storage = std::mem::zeroed();
    let mut len: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            receiver,
            ((&mut local as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((len) == (expected)));
    len = (0 as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            receiver,
            ((&mut local as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((len) == (expected)));
    assert!(((libc::bind(receiver, (addr).cast_const(), expected)) == (0)));
    len = (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            receiver,
            ((&mut local as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((len) == (expected)));
    assert!(
        ((libc::connect(
            sender,
            ((&mut local as *mut libc::sockaddr_storage) as *mut libc::sockaddr).cast_const(),
            len
        )) == (0))
    );
    let mut peer: libc::sockaddr_storage = std::mem::zeroed();
    len = (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getpeername(
            sender,
            ((&mut peer as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((len) == (expected)));
    assert!(
        (({
            let mut __tmp_0 = { (expected as usize) };
            let sa = core::slice::from_raw_parts(
                ((&mut local as *mut libc::sockaddr_storage) as *const ::libc::c_void) as *const u8,
                __tmp_0,
            );
            let sb = core::slice::from_raw_parts(
                ((&mut peer as *mut libc::sockaddr_storage) as *const ::libc::c_void) as *const u8,
                __tmp_0,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    let mut sent: libc::c_char = (('x' as i32) as libc::c_char);
    let mut received: libc::c_char = (0 as libc::c_char);
    assert!(
        ((libc::send(
            sender,
            ((&mut sent as *mut libc::c_char) as *const ::libc::c_void),
            1_usize,
            0
        )) == (1_isize))
    );
    let mut from: libc::sockaddr_storage = std::mem::zeroed();
    len = (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::recvfrom(
            receiver,
            ((&mut received as *mut libc::c_char) as *mut ::libc::c_void),
            1_usize,
            0,
            ((&mut from as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (1_isize))
    );
    assert!(((len) == (expected)));
    assert!(((received as i32) == (sent as i32)));
    len = (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            sender,
            ((&mut local as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((len) == (expected)));
    assert!(
        (({
            let mut __tmp_1 = { (expected as usize) };
            let sa = core::slice::from_raw_parts(
                ((&mut local as *mut libc::sockaddr_storage) as *const ::libc::c_void) as *const u8,
                __tmp_1,
            );
            let sb = core::slice::from_raw_parts(
                ((&mut from as *mut libc::sockaddr_storage) as *const ::libc::c_void) as *const u8,
                __tmp_1,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    received = (0 as libc::c_char);
    assert!(
        ((libc::send(
            sender,
            ((&mut sent as *mut libc::c_char) as *const ::libc::c_void),
            1_usize,
            0
        )) == (1_isize))
    );
    assert!(
        ((libc::recvfrom(
            receiver,
            ((&mut received as *mut libc::c_char) as *mut ::libc::c_void),
            1_usize,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut()
        )) == (1_isize))
    );
    assert!(((received as i32) == (sent as i32)));
    received = (0 as libc::c_char);
    len = (77 as libc::socklen_t);
    assert!(
        ((libc::send(
            sender,
            ((&mut sent as *mut libc::c_char) as *const ::libc::c_void),
            1_usize,
            0
        )) == (1_isize))
    );
    assert!(
        ((libc::recvfrom(
            receiver,
            ((&mut received as *mut libc::c_char) as *mut ::libc::c_void),
            1_usize,
            0,
            std::ptr::null_mut(),
            (&mut len as *mut libc::socklen_t)
        )) == (1_isize))
    );
    assert!(((len) == (77 as libc::socklen_t)));
    assert!(((received as i32) == (sent as i32)));
    assert!(((libc::close(sender)) == (0)));
    assert!(((libc::close(receiver)) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut in_: ::libc::sockaddr_in = std::mem::zeroed();
    {
        let mut __tmp_2 = { ((&mut in_ as *mut ::libc::sockaddr_in) as *mut ::libc::c_void) };
        let byte_0 = __tmp_2 as *mut u8;
        for offset in 0..::std::mem::size_of::<::libc::sockaddr_in>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        __tmp_2
    };
    in_.sin_family = (2 as libc::sa_family_t);
    in_.sin_addr.s_addr = ((u32::to_be(2130706433_u32)) as ::libc::in_addr_t);
    (unsafe {
        let _addr: *mut libc::sockaddr =
            ((&mut in_ as *mut ::libc::sockaddr_in) as *mut libc::sockaddr);
        let _expected: libc::socklen_t =
            (::std::mem::size_of::<::libc::sockaddr_in>() as libc::socklen_t);
        check_name_0(_addr, _expected)
    });
    let mut in6: ::libc::sockaddr_in6 = std::mem::zeroed();
    let mut fd: i32 = libc::socket(libc::AF_INET6, libc::SOCK_DGRAM, 0);
    assert!(((fd) >= (0)));
    let mut len: libc::socklen_t =
        (::std::mem::size_of::<::libc::sockaddr_in6>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            fd,
            ((&mut in6 as *mut ::libc::sockaddr_in6) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((len as usize) == (::std::mem::size_of::<::libc::sockaddr_in6>())));
    assert!(((libc::close(fd)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
