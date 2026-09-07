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
            Ptr::from_string_literal(b"x")
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(1_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 1_isize) as i32)
            != 0)
    );
    let pfd: Value<Box<[libcc2rs::Pollfd]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| Default::default())
            .collect::<Box<[libcc2rs::Pollfd]>>(),
    ));
    (*(*pfd.borrow())[(0) as usize].fd.borrow_mut()) = (*fds.borrow())[(0) as usize];
    (*(*pfd.borrow())[(0) as usize].events.borrow_mut()) = 1_i16;
    (*(*pfd.borrow())[(0) as usize].revents.borrow_mut()) = 0_i16;
    (*(*pfd.borrow())[(1) as usize].fd.borrow_mut()) = -1_i32;
    (*(*pfd.borrow())[(1) as usize].events.borrow_mut()) = 1_i16;
    (*(*pfd.borrow())[(1) as usize].revents.borrow_mut()) = 42_i16;
    assert!(
        ((({
            let __p = (pfd.as_pointer() as Ptr<libcc2rs::Pollfd>).clone();
            let __timeout = match nix::poll::PollTimeout::try_from(0) {
                Ok(__t) => __t,
                Err(_) => panic!("poll: unsupported timeout {}", 0),
            };
            let mut __idx = Vec::new();
            let mut __wanted = Vec::new();
            let mut __events = Vec::new();
            for __i in 0..((2 as ::libc::nfds_t) as usize) {
                let (__fd, __ev) = __p
                    .offset(__i)
                    .with(|__e| (*__e.fd.borrow(), *__e.events.borrow()));
                __p.offset(__i)
                    .with_mut(|__e| *__e.revents.borrow_mut() = 0);
                if __fd >= 0 {
                    __idx.push(__i);
                    __wanted.push(__fd);
                    __events.push(__ev);
                }
            }
            FdRegistry::with_fds(&__wanted, |__b| {
                let mut __pfds: Vec<nix::poll::PollFd> = __b
                    .iter()
                    .zip(&__events)
                    .map(|(&__fd, &__ev)| {
                        nix::poll::PollFd::new(__fd, nix::poll::PollFlags::from_bits_truncate(__ev))
                    })
                    .collect();
                match nix::poll::poll(&mut __pfds, __timeout) {
                    Ok(__count) => {
                        for (__slot, &__i) in __pfds.iter().zip(&__idx) {
                            let __rev = match __slot.revents() {
                                Some(__r) => __r.bits(),
                                None => 0,
                            };
                            __p.offset(__i)
                                .with_mut(|__e| *__e.revents.borrow_mut() = __rev);
                        }
                        __count
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            })
        } == 1) as i32)
            != 0)
    );
    assert!(
        ((((((*(*pfd.borrow())[(0) as usize].revents.borrow()) as i32) & 1) != 0) as i32) != 0)
    );
    assert!((((((*(*pfd.borrow())[(1) as usize].revents.borrow()) as i32) == 0) as i32) != 0));
    let ch: Value<u8> = <Value<u8>>::default();
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((ch.as_pointer()) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(1_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 1_isize) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    return 0;
}
