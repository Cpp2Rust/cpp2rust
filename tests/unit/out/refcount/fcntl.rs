extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fds: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>()));
    assert!(
        (((match ({ (fds.as_pointer() as Ptr::<i32>) },) {
            (mut __tmp_0,) => {
                match nix::unistd::pipe() {
                    Ok((__r, __w)) => {
                        __tmp_0.write(FdRegistry::register(__r));
                        elem!(__tmp_0, 1).write(FdRegistry::register(__w));
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            }
        } == 0) as i32)
            != 0)
    );
    let mut flags: i32 = match ({ (*fds.borrow())[(0) as usize] },) {
        (mut __tmp_1,) => {
            let __res = match 3 {
                ::libc::F_GETFL => FdRegistry::with_fd(__tmp_1, |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                        &Box::<[VaArg]>::from([(0).into()])[0],
                    ));
                    FdRegistry::with_fd(__tmp_1, |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_GETFD => FdRegistry::with_fd(__tmp_1, |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                }),
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                        &Box::<[VaArg]>::from([(0).into()])[0],
                    ));
                    FdRegistry::with_fd(__tmp_1, |__fd| {
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
        }
    };
    assert!((((flags >= 0) as i32) != 0));
    assert!(((((flags & ::libc::O_NONBLOCK) == 0) as i32) != 0));
    assert!(
        ({
            (((match ({ (*fds.borrow())[(0) as usize] },) {
                (mut __tmp_2,) => {
                    let __res = match 4 {
                        ::libc::F_GETFL => FdRegistry::with_fd(__tmp_2, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                        }),
                        ::libc::F_SETFL => {
                            let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(flags | ::libc::O_NONBLOCK).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_2, |__fd| {
                                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                            })
                        }
                        ::libc::F_GETFD => FdRegistry::with_fd(__tmp_2, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                        }),
                        ::libc::F_SETFD => {
                            let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(flags | ::libc::O_NONBLOCK).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_2, |__fd| {
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
                }
            } == 0) as i32)
                != 0)
        })
    );
    flags = match ({ (*fds.borrow())[(0) as usize] },) {
        (mut __tmp_3,) => {
            let __res = match 3 {
                ::libc::F_GETFL => FdRegistry::with_fd(__tmp_3, |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                }),
                ::libc::F_SETFL => {
                    let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                        &Box::<[VaArg]>::from([(0).into()])[0],
                    ));
                    FdRegistry::with_fd(__tmp_3, |__fd| {
                        nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                    })
                }
                ::libc::F_GETFD => FdRegistry::with_fd(__tmp_3, |__fd| {
                    nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                }),
                ::libc::F_SETFD => {
                    let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                        &Box::<[VaArg]>::from([(0).into()])[0],
                    ));
                    FdRegistry::with_fd(__tmp_3, |__fd| {
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
        }
    };
    assert!(((((flags & ::libc::O_NONBLOCK) != 0) as i32) != 0));
    let b: Value<i8> = Rc::new(RefCell::new(0_i8));
    assert!(
        (((match ({ ((b.as_pointer()) as Ptr<i8>).to_any() },) {
            (mut __tmp_4,) => {
                match FdRegistry::with_fd(({ (*fds.borrow())[(0) as usize] }), |__fd| {
                    __tmp_4
                        .reinterpret_cast::<u8>()
                        .with_slice_mut(1_usize, |__buf| nix::unistd::read(__fd, __buf))
                }) {
                    Ok(__n) => __n as isize,
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            }
        } == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(
        ({
            ((((match ({ (*fds.borrow())[(0) as usize] },) {
                (mut __tmp_5,) => {
                    let __res = match 1 {
                        ::libc::F_GETFL => FdRegistry::with_fd(__tmp_5, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                        }),
                        ::libc::F_SETFL => {
                            let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(0).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_5, |__fd| {
                                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                            })
                        }
                        ::libc::F_GETFD => FdRegistry::with_fd(__tmp_5, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                        }),
                        ::libc::F_SETFD => {
                            let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(0).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_5, |__fd| {
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
                }
            } & 1)
                == 0) as i32)
                != 0)
        })
    );
    assert!(
        ({
            (((match ({ (*fds.borrow())[(0) as usize] },) {
                (mut __tmp_6,) => {
                    let __res = match 2 {
                        ::libc::F_GETFL => FdRegistry::with_fd(__tmp_6, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                        }),
                        ::libc::F_SETFL => {
                            let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(1).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_6, |__fd| {
                                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                            })
                        }
                        ::libc::F_GETFD => FdRegistry::with_fd(__tmp_6, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                        }),
                        ::libc::F_SETFD => {
                            let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(1).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_6, |__fd| {
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
                }
            } == 0) as i32)
                != 0)
        })
    );
    assert!(
        ({
            ((((match ({ (*fds.borrow())[(0) as usize] },) {
                (mut __tmp_7,) => {
                    let __res = match 1 {
                        ::libc::F_GETFL => FdRegistry::with_fd(__tmp_7, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
                        }),
                        ::libc::F_SETFL => {
                            let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(0).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_7, |__fd| {
                                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
                            })
                        }
                        ::libc::F_GETFD => FdRegistry::with_fd(__tmp_7, |__fd| {
                            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
                        }),
                        ::libc::F_SETFD => {
                            let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(
                                &Box::<[VaArg]>::from([(0).into()])[0],
                            ));
                            FdRegistry::with_fd(__tmp_7, |__fd| {
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
                }
            } & 1)
                != 0) as i32)
                != 0)
        })
    );
    assert!((((FdRegistry::close(({ (*fds.borrow())[(0) as usize] })) == 0) as i32) != 0));
    assert!((((FdRegistry::close(({ (*fds.borrow())[(1) as usize] })) == 0) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
