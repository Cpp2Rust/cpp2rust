extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_from_0(
    mut receiver: i32,
    mut sender: i32,
    mut expected: *mut libc::sockaddr_storage,
    mut expected_len: libc::socklen_t,
) {
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
    {
        let mut __tmp_0 = { ((&mut from as *mut libc::sockaddr_storage) as *mut ::libc::c_void) };
        let byte_0 = __tmp_0 as *mut u8;
        for offset in 0..::std::mem::size_of::<libc::sockaddr_storage>() {
            *byte_0.offset(offset as isize) = 165 as u8;
        }
        __tmp_0
    };
    let mut len: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
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
    assert!(((len) == (expected_len)));
    assert!(((received as i32) == (sent as i32)));
    assert!(
        (({
            let mut __tmp_1 = { (len as usize) };
            let sa = core::slice::from_raw_parts(
                ((&mut from as *mut libc::sockaddr_storage) as *const ::libc::c_void) as *const u8,
                __tmp_1,
            );
            let sb = core::slice::from_raw_parts(
                (expected as *const ::libc::c_void) as *const u8,
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
    {
        let mut i: u32 = (len as u32);
        'loop_: while ((i as usize) < (::std::mem::size_of::<libc::sockaddr_storage>())) {
            'for_body: {
                assert!(
                    (((*((&mut from as *mut libc::sockaddr_storage) as *mut u8)
                        .offset((i) as isize)) as i32)
                        == (165))
                );
            }
            i.prefix_inc();
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut receiver: i32 = libc::socket(1, libc::SOCK_DGRAM, 0);
    let mut sender: i32 = libc::socket(1, libc::SOCK_DGRAM, 0);
    assert!(((receiver) >= (0)));
    assert!(((sender) >= (0)));
    let mut addr: libc::sockaddr_storage = std::mem::zeroed();
    let mut len: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            receiver,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    let mut un: *mut libc::sockaddr_un =
        ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr_un);
    {
        let mut __tmp_2 = { ((*un).sun_path.as_mut_ptr() as *mut ::libc::c_void) };
        if ::std::mem::size_of::<[libc::c_char; 18]>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (c"recvfrom-receiver".as_ptr().cast_mut() as *const ::libc::c_void),
                __tmp_2,
                ::std::mem::size_of::<[libc::c_char; 18]>(),
            )
        }
        __tmp_2
    };
    len = ((((::std::mem::offset_of!(libc::sockaddr_un, sun_path) as usize)
        .wrapping_add(libc::strlen(((*un).sun_path.as_mut_ptr()).cast_const())))
    .wrapping_add(1_usize)) as libc::socklen_t);
    assert!(
        ((libc::bind(
            receiver,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr).cast_const(),
            len
        )) == (0))
    );
    assert!(
        ((libc::connect(
            sender,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr).cast_const(),
            len
        )) == (0))
    );
    let mut received: libc::c_char = (0 as libc::c_char);
    assert!(
        ((libc::send(
            sender,
            (c"x".as_ptr().cast_mut() as *const ::libc::c_void),
            1_usize,
            0
        )) == (1_isize))
    );
    let mut from: libc::sockaddr_storage = std::mem::zeroed();
    let mut from_len: libc::socklen_t =
        (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::recvfrom(
            receiver,
            ((&mut received as *mut libc::c_char) as *mut ::libc::c_void),
            1_usize,
            0,
            ((&mut from as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut from_len as *mut libc::socklen_t)
        )) == (1_isize))
    );
    assert!(((received as i32) == ('x' as i32)));
    assert!(
        ((libc::send(
            sender,
            (c"x".as_ptr().cast_mut() as *const ::libc::c_void),
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
    assert!(((received as i32) == ('x' as i32)));
    received = (0 as libc::c_char);
    len = (77 as libc::socklen_t);
    assert!(
        ((libc::send(
            sender,
            (c"x".as_ptr().cast_mut() as *const ::libc::c_void),
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
    assert!(((received as i32) == ('x' as i32)));
    {
        let mut __tmp_3 = { ((*un).sun_path.as_mut_ptr() as *mut ::libc::c_void) };
        if ::std::mem::size_of::<[libc::c_char; 16]>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (c"recvfrom-sender".as_ptr().cast_mut() as *const ::libc::c_void),
                __tmp_3,
                ::std::mem::size_of::<[libc::c_char; 16]>(),
            )
        }
        __tmp_3
    };
    len = ((((::std::mem::offset_of!(libc::sockaddr_un, sun_path) as usize)
        .wrapping_add(libc::strlen(((*un).sun_path.as_mut_ptr()).cast_const())))
    .wrapping_add(1_usize)) as libc::socklen_t);
    assert!(
        ((libc::bind(
            sender,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr).cast_const(),
            len
        )) == (0))
    );
    len = (::std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t);
    assert!(
        ((libc::getsockname(
            sender,
            ((&mut addr as *mut libc::sockaddr_storage) as *mut libc::sockaddr),
            (&mut len as *mut libc::socklen_t)
        )) == (0))
    );
    (unsafe {
        check_from_0(
            receiver,
            sender,
            (&mut addr as *mut libc::sockaddr_storage),
            len,
        )
    });
    received = (0 as libc::c_char);
    assert!(
        ((libc::send(
            sender,
            (c"x".as_ptr().cast_mut() as *const ::libc::c_void),
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
    assert!(((received as i32) == ('x' as i32)));
    received = (0 as libc::c_char);
    len = (77 as libc::socklen_t);
    assert!(
        ((libc::send(
            sender,
            (c"x".as_ptr().cast_mut() as *const ::libc::c_void),
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
    assert!(((received as i32) == ('x' as i32)));
    assert!(((libc::close(sender)) == (0)));
    assert!(((libc::close(receiver)) == (0)));
    assert!(((libc::unlink((c"recvfrom-sender".as_ptr().cast_mut()).cast_const())) == (0)));
    assert!(((libc::unlink((c"recvfrom-receiver".as_ptr().cast_mut()).cast_const())) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
