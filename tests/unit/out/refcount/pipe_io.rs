extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(1) as usize], |__fd| {
            Ptr::from_string_literal(b"ab")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(2_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 2_isize) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(::std::mem::size_of::<[u8; 4]>(), |__buf| {
                    nix::unistd::read(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 2_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"ab").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(::std::mem::size_of::<[u8; 4]>(), |__buf| {
                    nix::unistd::read(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0_isize) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    return 0;
}
