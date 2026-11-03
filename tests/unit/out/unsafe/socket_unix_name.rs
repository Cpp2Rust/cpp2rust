extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_name_0(mut fd: i32, mut path: *const libc::c_char) {
    let mut full: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    match ({ ((&mut full as *mut libc::sockaddr_storage) as *mut ::libc::c_void) },) {
        (mut __tmp_0,) => {
            let byte_0 = __tmp_0 as *mut u8;
            for offset in 0..::std::mem::size_of::<libc::sockaddr_storage>() {
                *byte_0.offset(offset as isize) = 165 as u8;
            }
            __tmp_0
        }
    };
    let mut actual: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((((libc::getsockname(
            fd,
            ((&mut full as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut actual as *mut libc::socklen_t)
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((full.ss_family as i32) == (1)) as i32) != 0));
    assert!(
        ((((actual as usize) >= (::std::mem::offset_of!(libc::sockaddr_un, sun_path))) as i32)
            != 0)
    );
    assert!(
        ((((actual as usize) < (::std::mem::size_of::<libc::sockaddr_storage>())) as i32) != 0)
    );
    if !(path).is_null() {
        assert!(
            ((((match ({ libc::strlen(path) },) {
                (mut __tmp_1,) => {
                    {
                        let sa = core::slice::from_raw_parts(
                            ((*((&mut full as *mut libc::sockaddr_storage)
                                as *mut libc::sockaddr_un))
                                .sun_path
                                .as_mut_ptr() as *const ::libc::c_void)
                                as *const u8,
                            __tmp_1 as usize,
                        );
                        let sb = core::slice::from_raw_parts(
                            (path as *const ::libc::c_void) as *const u8,
                            __tmp_1 as usize,
                        );
                        let mut diff = 0_i32;
                        for (x, y) in sa.iter().zip(sb.iter()) {
                            if x != y {
                                diff = (*x as i32) - (*y as i32);
                                break;
                            }
                        }
                        diff
                    }
                }
            }) == (0)) as i32)
                != 0)
        );
    }
    let mut capacities: [libc::socklen_t; 6] = [
        (0 as libc::socklen_t),
        (1 as libc::socklen_t),
        (2 as libc::socklen_t),
        (actual).wrapping_sub((1 as libc::socklen_t)),
        actual,
        (actual).wrapping_add((1 as libc::socklen_t)),
    ];
    let mut i: u32 = 0_u32;
    'loop_: while ((((i as usize)
        < ((::std::mem::size_of::<[libc::socklen_t; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::socklen_t>() as usize))))
        as i32)
        != 0)
    {
        let mut out: [u8; 136] = [0_u8; 136];
        match ({ (out.as_mut_ptr() as *mut ::libc::c_void) },) {
            (mut __tmp_2,) => {
                let byte_0 = __tmp_2 as *mut u8;
                for offset in 0..::std::mem::size_of::<[u8; 136]>() {
                    *byte_0.offset(offset as isize) = 165 as u8;
                }
                __tmp_2
            }
        };
        let mut len: libc::socklen_t = capacities[(i) as usize];
        assert!(
            ((((libc::getsockname(
                fd,
                (out.as_mut_ptr() as *mut libc::sockaddr),
                (&mut len as *mut libc::socklen_t)
            )) == (0)) as i32)
                != 0)
        );
        assert!(((((len) == (actual)) as i32) != 0));
        let mut copied: u32 = (if ((((capacities[(i) as usize]) < (actual)) as i32) != 0) {
            capacities[(i) as usize]
        } else {
            actual
        } as u32);
        assert!(
            ((((match ({ (copied as usize) },) {
                (mut __tmp_3,) => {
                    {
                        let sa = core::slice::from_raw_parts(
                            (out.as_mut_ptr() as *const ::libc::c_void) as *const u8,
                            __tmp_3 as usize,
                        );
                        let sb = core::slice::from_raw_parts(
                            ((&mut full as *mut libc::sockaddr_storage) as *const ::libc::c_void)
                                as *const u8,
                            __tmp_3 as usize,
                        );
                        let mut diff = 0_i32;
                        for (x, y) in sa.iter().zip(sb.iter()) {
                            if x != y {
                                diff = (*x as i32) - (*y as i32);
                                break;
                            }
                        }
                        diff
                    }
                }
            }) == (0)) as i32)
                != 0)
        );
        let mut j: u32 = copied;
        'loop_: while ((((j as usize) < (::std::mem::size_of::<[u8; 136]>())) as i32) != 0) {
            assert!(((((out[(j) as usize] as i32) == (165)) as i32) != 0));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fd: i32 = libc::socket(1, libc::SOCK_DGRAM, 0);
    assert!(((((fd) >= (0)) as i32) != 0));
    (unsafe { check_name_0(fd, std::ptr::null()) });
    let mut addr: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    match ({ ((&mut addr as *mut libc::sockaddr_storage) as *mut ::libc::c_void) },) {
        (mut __tmp_4,) => {
            let byte_0 = __tmp_4 as *mut u8;
            for offset in 0..::std::mem::size_of::<libc::sockaddr_storage>() {
                *byte_0.offset(offset as isize) = 0 as u8;
            }
            __tmp_4
        }
    };
    let mut len: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((((libc::getsockname(
            fd,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0)) as i32)
            != 0)
    );
    let mut un: *mut libc::sockaddr_un =
        ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr_un);
    match ({ ((*un).sun_path.as_mut_ptr() as *mut ::libc::c_void) },) {
        (mut __tmp_5,) => {
            if ::std::mem::size_of::<[libc::c_char; 17]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (c"socket-unix-name".as_ptr().cast_mut() as *const ::libc::c_void),
                    __tmp_5,
                    ::std::mem::size_of::<[libc::c_char; 17]>() as usize,
                )
            }
            __tmp_5
        }
    };
    assert!(
        ((((libc::bind(
            fd,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr).cast_const(),
            (((::std::mem::offset_of!(libc::sockaddr_un, sun_path) as usize)
                .wrapping_add(libc::strlen(((*un).sun_path.as_mut_ptr()).cast_const())))
                as libc::socklen_t)
        )) == (0)) as i32)
            != 0)
    );
    (unsafe { check_name_0(fd, ((*un).sun_path.as_mut_ptr()).cast_const()) });
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(((*un).sun_path.as_mut_ptr()).cast_const())) == (0)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
