// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd};

pub struct FdRegistry {
    fds: Vec<Option<OwnedFd>>,
}

thread_local! {
    static FD_REGISTRY: RefCell<FdRegistry> = const { RefCell::new(FdRegistry { fds: Vec::new() }) };
}

impl FdRegistry {
    pub fn register(fd: OwnedFd) -> i32 {
        let raw = fd.as_raw_fd();
        FD_REGISTRY.with(|r| {
            let fds = &mut r.borrow_mut().fds;
            let idx = raw as usize;
            if fds.len() <= idx {
                fds.resize_with(idx + 1, || None);
            }
            assert!(fds[idx].is_none(), "ub: fd registry collision on fd {raw}");
            fds[idx] = Some(fd);
        });
        raw
    }

    pub fn with_fd<R>(fd: i32, f: impl FnOnce(BorrowedFd<'_>) -> R) -> R {
        FD_REGISTRY.with(|r| {
            let reg = r.borrow();
            let owned = reg
                .fds
                .get(fd as usize)
                .and_then(|slot| slot.as_ref())
                .unwrap_or_else(|| panic!("ub: bad fd {fd}"));
            f(owned.as_fd())
        })
    }

    pub fn close(fd: i32) -> i32 {
        FD_REGISTRY.with(|r| {
            r.borrow_mut()
                .fds
                .get_mut(fd as usize)
                .and_then(|slot| slot.take())
                .unwrap_or_else(|| panic!("ub: bad fd {fd}"))
        });
        0
    }
}
