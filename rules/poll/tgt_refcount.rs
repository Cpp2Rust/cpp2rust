// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Pollfd {
    Default::default()
}

fn f1(a0: Ptr<Pollfd>, a1: u64, a2: i32) -> i32 {
    let __p = a0.clone();
    let __timeout = match nix::poll::PollTimeout::try_from(a2) {
        Ok(__t) => __t,
        Err(_) => panic!("poll: unsupported timeout {}", a2),
    };
    let mut __idx = Vec::new();
    let mut __wanted = Vec::new();
    let mut __events = Vec::new();
    for __i in 0..(a1 as usize) {
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
}
