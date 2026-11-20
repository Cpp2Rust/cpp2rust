extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_buf_0(
    mut fd: i32,
    mut buf: *mut u8,
    mut size: u32,
    mut cap: libc::socklen_t,
    mut full: *const ::libc::c_void,
    mut actual: libc::socklen_t,
) {
    {
        let mut __tmp_0 = { (buf as *mut ::libc::c_void) };
        let byte_0 = __tmp_0 as *mut u8;
        for offset in 0..(size as usize) {
            *byte_0.offset(offset as isize) = 165 as u8;
        }
        __tmp_0
    };
    let mut len: libc::socklen_t = cap;
    assert!(
        ((libc::getsockname(
            fd,
            (buf as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    let mut copied: u32 = (if ((cap) < (actual)) { cap } else { actual } as u32);
    assert!(
        (({
            let mut __tmp_1 = { (copied as usize) };
            let sa =
                core::slice::from_raw_parts((buf as *const ::libc::c_void) as *const u8, __tmp_1);
            let sb = core::slice::from_raw_parts(full as *const u8, __tmp_1);
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
    {
        let mut j: u32 = copied;
        'loop_: while ((j) < (size)) {
            assert!((((*buf.offset((j) as isize)) as i32) == (165)));
            j.prefix_inc();
        }
    }
}
pub unsafe fn check_name_1(mut family: i32) {
    let mut fd: i32 = libc::socket(family, libc::SOCK_DGRAM, 0);
    assert!(((fd) >= (0)));
    let mut full: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    {
        let mut __tmp_2 = { ((&mut full as *mut libc::sockaddr_storage) as *mut ::libc::c_void) };
        let byte_0 = __tmp_2 as *mut u8;
        for offset in 0..::std::mem::size_of::<libc::sockaddr_storage>() {
            *byte_0.offset(offset as isize) = 165 as u8;
        }
        __tmp_2
    };
    let mut len: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            fd,
            ((&mut full as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    assert!(((full.ss_family as i32) == (family)));
    let mut actual: libc::socklen_t = (if ((family) == (libc::AF_INET)) {
        ::std::mem::size_of::<::libc::sockaddr_in>()
    } else {
        ::std::mem::size_of::<::libc::sockaddr_in6>()
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
            < ((::std::mem::size_of::<[libc::socklen_t; 6]>() as usize)
                .wrapping_div((::std::mem::size_of::<libc::socklen_t>() as usize))))
        {
            let mut out: [u8; 136] = [0_u8; 136];
            (unsafe {
                let _buf: *mut u8 = out.as_mut_ptr();
                let _size: u32 = (::std::mem::size_of::<[u8; 136]>() as u32);
                check_buf_0(
                    fd,
                    _buf,
                    _size,
                    capacities[(i) as usize],
                    ((&mut full as *mut libc::sockaddr_storage) as *const ::libc::c_void),
                    actual,
                )
            });
            let mut size: u32 = (if (capacities[(i) as usize] != 0) {
                capacities[(i) as usize]
            } else {
                (1 as libc::socklen_t)
            } as u32);
            let mut small: *mut u8 = (libcc2rs::malloc_unsafe((size as usize)) as *mut u8);
            assert!(!(small).is_null());
            (unsafe {
                check_buf_0(
                    fd,
                    small,
                    size,
                    capacities[(i) as usize],
                    ((&mut full as *mut libc::sockaddr_storage) as *const ::libc::c_void),
                    actual,
                )
            });
            libcc2rs::free_unsafe((small as *mut ::libc::c_void));
            i.prefix_inc();
        }
    }
    assert!(((libc::close(fd)) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { check_name_1(libc::AF_INET) });
    (unsafe { check_name_1(libc::AF_INET6) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
