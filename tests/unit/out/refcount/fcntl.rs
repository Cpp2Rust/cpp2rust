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
    let flags: Value<i32> = Rc::new(RefCell::new({
        let __res = match 3 {
            ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
            }),
            ::libc::F_SETFL => {
                let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                })
            }
            ::libc::F_GETFD => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
            }),
            ::libc::F_SETFD => {
                let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                })
            }
            __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
        };
        match __res {
            Ok(__r) => __r,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*flags.borrow()) >= 0) as i32) != 0));
    assert!((((((*flags.borrow()) & ::libc::O_NONBLOCK) == 0) as i32) != 0));
    assert!(
        ((({
            let __res = match 4 {
                ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                        &&[((*flags.borrow()) | ::libc::O_NONBLOCK).into()][0],
                    ));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_GETFD => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                }),
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                        &&[((*flags.borrow()) | ::libc::O_NONBLOCK).into()][0],
                    ));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                    })
                }
                __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
            };
            match __res {
                Ok(__r) => __r,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    (*flags.borrow_mut()) = {
        let __res = match 3 {
            ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
            }),
            ::libc::F_SETFL => {
                let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                })
            }
            ::libc::F_GETFD => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
            }),
            ::libc::F_SETFD => {
                let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                })
            }
            __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
        };
        match __res {
            Ok(__r) => __r,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!((((((*flags.borrow()) & ::libc::O_NONBLOCK) != 0) as i32) != 0));
    let b: Value<u8> = <Value<u8>>::default();
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((b.as_pointer()) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(1_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let __res = match 1 {
                ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_GETFD => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                }),
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                    })
                }
                __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
            };
            match __res {
                Ok(__r) => __r,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } & 1)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __res = match 2 {
                ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(1).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_GETFD => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                }),
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(1).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                    })
                }
                __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
            };
            match __res {
                Ok(__r) => __r,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((({
            let __res = match 1 {
                ::libc::F_GETFL => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_GETFD => FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                }),
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&&[(0).into()][0]));
                    FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
                    })
                }
                __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
            };
            match __res {
                Ok(__r) => __r,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } & 1)
            != 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    return 0;
}
