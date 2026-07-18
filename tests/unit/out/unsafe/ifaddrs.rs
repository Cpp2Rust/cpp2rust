extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut list: *mut libc::ifaddrs = std::ptr::null_mut();
    assert!(((((libc::getifaddrs((&mut list as *mut *mut libc::ifaddrs))) == (0)) as i32) != 0));
    assert!((((!((list).is_null())) as i32) != 0));
    let mut found_loopback: i32 = 0;
    let mut ifa: *mut libc::ifaddrs = std::ptr::null_mut();
    ifa = list;
    'loop_: while (((!((ifa).is_null())) as i32) != 0) {
        assert!((((!(((*ifa).ifa_name).is_null())) as i32) != 0));
        if (((((*ifa).ifa_addr).is_null()) as i32) != 0) {
            ifa = (*ifa).ifa_next;
            continue 'loop_;
        }
        if (((((*(*ifa).ifa_addr).sa_family as i32) != (libc::AF_INET)) as i32) != 0) {
            ifa = (*ifa).ifa_next;
            continue 'loop_;
        }
        let mut sin: *mut ::libc::sockaddr_in = ((*ifa).ifa_addr as *mut ::libc::sockaddr_in);
        let mut lo_be: [u8; 4] = [127_u8, 0_u8, 0_u8, 1_u8];
        if (((({
            let sa = core::slice::from_raw_parts(
                ((&mut (*sin).sin_addr as *mut ::libc::in_addr) as *const ::libc::in_addr
                    as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (lo_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
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
        {
            found_loopback = 1;
            assert!((((((*ifa).ifa_flags) != (0_u32)) as i32) != 0));
            assert!((((!(((*ifa).ifa_netmask).is_null())) as i32) != 0));
            let mut mask: *mut ::libc::sockaddr_in =
                ((*ifa).ifa_netmask as *mut ::libc::sockaddr_in);
            let mut mask_be: [u8; 4] = [255_u8, 0_u8, 0_u8, 0_u8];
            assert!(
                (((({
                    let sa = core::slice::from_raw_parts(
                        ((&mut (*mask).sin_addr as *mut ::libc::in_addr) as *const ::libc::in_addr
                            as *const ::libc::c_void) as *const u8,
                        4_usize as usize,
                    );
                    let sb = core::slice::from_raw_parts(
                        (mask_be.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
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
            assert!(
                ((((libc::if_nametoindex(((*ifa).ifa_name).cast_const())) > (0_u32)) as i32) != 0)
            );
        }
        ifa = (*ifa).ifa_next;
    }
    assert!((found_loopback != 0));
    libc::freeifaddrs(list);
    assert!(
        ((((libc::if_nametoindex((c"cpp2rust_no_such_if".as_ptr().cast_mut()).cast_const()))
            == (0_u32)) as i32)
            != 0)
    );
    return 0;
}
