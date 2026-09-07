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
    let rset: Value<CFdSet> = Rc::new(RefCell::new(Default::default()));
    (rset.as_pointer()).with_mut(|__s| __s.zero());
    (rset.as_pointer()).with_mut(|__s| __s.set((*fds.borrow())[(0) as usize]));
    let tv: Value<libcc2rs::Timeval> = Rc::new(RefCell::new(Default::default()));
    {
        ((tv.as_pointer()) as Ptr<libcc2rs::Timeval>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((tv.as_pointer()) as Ptr<libcc2rs::Timeval>)
            .to_any()
            .clone()
    };
    (*(*tv.borrow()).tv_sec.borrow_mut()) = 0_i64;
    assert!(
        ((({
            let __rp = (rset.as_pointer()).clone();
            let __wp = Ptr::<CFdSet>::null().clone();
            let __ep = Ptr::<CFdSet>::null().clone();
            let __tp = (tv.as_pointer()).clone();
            let __r_fds: Vec<i32> = match __rp.is_null() {
                true => Vec::new(),
                false => __rp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __w_fds: Vec<i32> = match __wp.is_null() {
                true => Vec::new(),
                false => __wp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __e_fds: Vec<i32> = match __ep.is_null() {
                true => Vec::new(),
                false => __ep.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let mut __wanted = Vec::new();
            __wanted.extend(&__r_fds);
            __wanted.extend(&__w_fds);
            __wanted.extend(&__e_fds);
            FdRegistry::with_fds(&__wanted, |__b| {
                let __rn = __r_fds.len();
                let __wn = __w_fds.len();
                let mut __rs = nix::sys::select::FdSet::new();
                let mut __ws = nix::sys::select::FdSet::new();
                let mut __es = nix::sys::select::FdSet::new();
                for __bfd in &__b[..__rn] {
                    __rs.insert(*__bfd);
                }
                for __bfd in &__b[__rn..__rn + __wn] {
                    __ws.insert(*__bfd);
                }
                for __bfd in &__b[__rn + __wn..] {
                    __es.insert(*__bfd);
                }
                let mut __tv = match __tp.is_null() {
                    true => None,
                    false => Some(__tp.with(|__t| {
                        nix::sys::time::TimeVal::new(
                            *__t.tv_sec.borrow() as _,
                            *__t.tv_usec.borrow() as _,
                        )
                    })),
                };
                match nix::sys::select::select(
                    ((*fds.borrow())[(0) as usize] + 1),
                    match __rp.is_null() {
                        true => None,
                        false => Some(&mut __rs),
                    },
                    match __wp.is_null() {
                        true => None,
                        false => Some(&mut __ws),
                    },
                    match __ep.is_null() {
                        true => None,
                        false => Some(&mut __es),
                    },
                    __tv.as_mut(),
                ) {
                    Ok(__n) => {
                        if !__rp.is_null() {
                            __rp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __r_fds.iter().zip(&__b[..__rn]) {
                                    if __rs.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__wp.is_null() {
                            __wp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __w_fds.iter().zip(&__b[__rn..__rn + __wn]) {
                                    if __ws.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__ep.is_null() {
                            __ep.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __e_fds.iter().zip(&__b[__rn + __wn..]) {
                                    if __es.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        match (__tp.is_null(), __tv.as_ref()) {
                            (false, Some(__t)) => __tp.with_mut(|__dst| {
                                *__dst.tv_sec.borrow_mut() = __t.tv_sec() as i64;
                                *__dst.tv_usec.borrow_mut() = __t.tv_usec() as i64;
                            }),
                            _ => {}
                        }
                        __n
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            })
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((!(if (rset.as_pointer()).with(|__s| __s.isset((*fds.borrow())[(0) as usize])) {
            1
        } else {
            0
        } != 0) as i32)
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
    (rset.as_pointer()).with_mut(|__s| __s.zero());
    (rset.as_pointer()).with_mut(|__s| __s.set((*fds.borrow())[(0) as usize]));
    (*(*tv.borrow()).tv_sec.borrow_mut()) = 1_i64;
    assert!(
        ((({
            let __rp = (rset.as_pointer()).clone();
            let __wp = Ptr::<CFdSet>::null().clone();
            let __ep = Ptr::<CFdSet>::null().clone();
            let __tp = (tv.as_pointer()).clone();
            let __r_fds: Vec<i32> = match __rp.is_null() {
                true => Vec::new(),
                false => __rp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __w_fds: Vec<i32> = match __wp.is_null() {
                true => Vec::new(),
                false => __wp.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let __e_fds: Vec<i32> = match __ep.is_null() {
                true => Vec::new(),
                false => __ep.with(|__s| {
                    (0..((*fds.borrow())[(0) as usize] + 1))
                        .filter(|&__fd| __s.isset(__fd))
                        .collect()
                }),
            };
            let mut __wanted = Vec::new();
            __wanted.extend(&__r_fds);
            __wanted.extend(&__w_fds);
            __wanted.extend(&__e_fds);
            FdRegistry::with_fds(&__wanted, |__b| {
                let __rn = __r_fds.len();
                let __wn = __w_fds.len();
                let mut __rs = nix::sys::select::FdSet::new();
                let mut __ws = nix::sys::select::FdSet::new();
                let mut __es = nix::sys::select::FdSet::new();
                for __bfd in &__b[..__rn] {
                    __rs.insert(*__bfd);
                }
                for __bfd in &__b[__rn..__rn + __wn] {
                    __ws.insert(*__bfd);
                }
                for __bfd in &__b[__rn + __wn..] {
                    __es.insert(*__bfd);
                }
                let mut __tv = match __tp.is_null() {
                    true => None,
                    false => Some(__tp.with(|__t| {
                        nix::sys::time::TimeVal::new(
                            *__t.tv_sec.borrow() as _,
                            *__t.tv_usec.borrow() as _,
                        )
                    })),
                };
                match nix::sys::select::select(
                    ((*fds.borrow())[(0) as usize] + 1),
                    match __rp.is_null() {
                        true => None,
                        false => Some(&mut __rs),
                    },
                    match __wp.is_null() {
                        true => None,
                        false => Some(&mut __ws),
                    },
                    match __ep.is_null() {
                        true => None,
                        false => Some(&mut __es),
                    },
                    __tv.as_mut(),
                ) {
                    Ok(__n) => {
                        if !__rp.is_null() {
                            __rp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __r_fds.iter().zip(&__b[..__rn]) {
                                    if __rs.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__wp.is_null() {
                            __wp.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __w_fds.iter().zip(&__b[__rn..__rn + __wn]) {
                                    if __ws.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        if !__ep.is_null() {
                            __ep.with_mut(|__s| {
                                __s.zero();
                                for (__fd, __bfd) in __e_fds.iter().zip(&__b[__rn + __wn..]) {
                                    if __es.contains(*__bfd) {
                                        __s.set(*__fd);
                                    }
                                }
                            });
                        }
                        match (__tp.is_null(), __tv.as_ref()) {
                            (false, Some(__t)) => __tp.with_mut(|__dst| {
                                *__dst.tv_sec.borrow_mut() = __t.tv_sec() as i64;
                                *__dst.tv_usec.borrow_mut() = __t.tv_usec() as i64;
                            }),
                            _ => {}
                        }
                        __n
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
        (if (rset.as_pointer()).with(|__s| __s.isset((*fds.borrow())[(0) as usize])) {
            1
        } else {
            0
        } != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    return 0;
}
