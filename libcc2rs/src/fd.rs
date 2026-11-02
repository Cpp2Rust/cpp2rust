// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd};

enum Fd {
    Owned(OwnedFd),
    Stdin(std::io::Stdin),
    Stdout(std::io::Stdout),
    Stderr(std::io::Stderr),
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> i32 {
        match self {
            Self::Owned(fd) => fd.as_raw_fd(),
            Self::Stdin(fd) => fd.as_raw_fd(),
            Self::Stdout(fd) => fd.as_raw_fd(),
            Self::Stderr(fd) => fd.as_raw_fd(),
        }
    }
}

impl AsFd for Fd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self {
            Self::Owned(fd) => fd.as_fd(),
            Self::Stdin(fd) => fd.as_fd(),
            Self::Stdout(fd) => fd.as_fd(),
            Self::Stderr(fd) => fd.as_fd(),
        }
    }
}

pub struct FdRegistry {
    fds: Vec<Option<Fd>>,
}

thread_local! {
    static FD_REGISTRY: RefCell<FdRegistry> = RefCell::new(FdRegistry {
        // Borrow the original standard descriptors, without closing them when
        // this thread exits. Only an explicit close releases these descriptors.
        fds: [
            Fd::Stdin(std::io::stdin()),
            Fd::Stdout(std::io::stdout()),
            Fd::Stderr(std::io::stderr()),
        ].into_iter().map(|fd| {
            // Check the number without borrowing a potentially closed descriptor.
            (unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFD) } >= 0).then_some(fd)
        }).collect(),
    });
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
            // The first open may reuse an initially closed standard descriptor
            // before the registry is initialized. Replace its borrowed entry.
            assert!(
                !matches!(fds[idx], Some(Fd::Owned(_))),
                "ub: fd registry collision on fd {raw}"
            );
            fds[idx] = Some(Fd::Owned(fd));
        });
        raw
    }

    fn lookup(&self, fd: i32) -> BorrowedFd<'_> {
        self.fds
            .get(fd as usize)
            .and_then(|slot| slot.as_ref())
            .unwrap_or_else(|| panic!("ub: bad fd {fd}"))
            .as_fd()
    }

    pub fn with_fd<R>(fd: i32, f: impl FnOnce(BorrowedFd<'_>) -> R) -> R {
        FD_REGISTRY.with(|r| {
            let reg = r.borrow();
            f(reg.lookup(fd))
        })
    }

    pub fn with_fds<R>(fds: &[i32], f: impl FnOnce(&[BorrowedFd<'_>]) -> R) -> R {
        FD_REGISTRY.with(|r| {
            let reg = r.borrow();
            let borrowed: Vec<BorrowedFd<'_>> = fds.iter().map(|&fd| reg.lookup(fd)).collect();
            f(&borrowed)
        })
    }

    pub fn close(fd: i32) -> i32 {
        let entry = FD_REGISTRY.with(|r| {
            r.borrow_mut()
                .fds
                .get_mut(fd as usize)
                .and_then(|slot| slot.take())
                .unwrap_or_else(|| panic!("ub: bad fd {fd}"))
        });
        match entry {
            Fd::Owned(fd) => drop(fd),
            fd => {
                if let Err(e) = nix::unistd::close(fd.as_raw_fd()) {
                    crate::cpp2rust_errno().write(e as i32);
                    return -1;
                }
            }
        }
        0
    }
}
