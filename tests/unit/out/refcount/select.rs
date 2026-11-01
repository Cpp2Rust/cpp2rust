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
    let rset: Value<CFdSet> = Rc::new(RefCell::new(Default::default()));
    (rset.as_pointer()).with_mut(|__s| __s.zero());
    match ({ (*fds.borrow())[(0) as usize] },) {
        (mut __tmp_1,) => {
            (rset.as_pointer()).with_mut(|__s| __s.set(__tmp_1));
        }
    };
    let tv: Value<libcc2rs::Timeval> = Rc::new(RefCell::new(Default::default()));
    match ({ ((tv.as_pointer()) as Ptr<libcc2rs::Timeval>).to_any() },) {
        (mut __tmp_2,) => {
            __tmp_2.memset((0) as u8, 16usize as usize);
            (__tmp_2).clone()
        }
    };
    (*tv.borrow_mut()).tv_sec = (0 as ::libc::time_t);
    assert!(
        (((match (
            { ((*fds.borrow())[(0) as usize] + 1) },
            { (rset.as_pointer()) },
            { (tv.as_pointer()) },
        ) {
            (mut __tmp_3, mut __tmp_4, mut __tmp_5) => {
                {
                    let __r_fds: Vec<i32> = match __tmp_4.is_null() {
                        true => Vec::new(),
                        false => __tmp_4
                            .with(|__s| (0..__tmp_3).filter(|&__fd| __s.isset(__fd)).collect()),
                    };
                    let __w_fds: Vec<i32> = match Ptr::<CFdSet>::null().is_null() {
                        true => Vec::new(),
                        false => Ptr::<CFdSet>::null()
                            .with(|__s| (0..__tmp_3).filter(|&__fd| __s.isset(__fd)).collect()),
                    };
                    let __e_fds: Vec<i32> = match Ptr::<CFdSet>::null().is_null() {
                        true => Vec::new(),
                        false => Ptr::<CFdSet>::null()
                            .with(|__s| (0..__tmp_3).filter(|&__fd| __s.isset(__fd)).collect()),
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
                        let mut __tv = match __tmp_5.is_null() {
                            true => None,
                            false => Some(__tmp_5.with(|__t| {
                                nix::sys::time::TimeVal::new(__t.tv_sec as _, __t.tv_usec as _)
                            })),
                        };
                        match nix::sys::select::select(
                            __tmp_3,
                            match __tmp_4.is_null() {
                                true => None,
                                false => Some(&mut __rs),
                            },
                            match Ptr::<CFdSet>::null().is_null() {
                                true => None,
                                false => Some(&mut __ws),
                            },
                            match Ptr::<CFdSet>::null().is_null() {
                                true => None,
                                false => Some(&mut __es),
                            },
                            __tv.as_mut(),
                        ) {
                            Ok(__n) => {
                                if !__tmp_4.is_null() {
                                    __tmp_4.with_mut(|__s| {
                                        __s.zero();
                                        for (__fd, __bfd) in __r_fds.iter().zip(&__b[..__rn]) {
                                            if __rs.contains(*__bfd) {
                                                __s.set(*__fd);
                                            }
                                        }
                                    });
                                }
                                if !Ptr::<CFdSet>::null().is_null() {
                                    Ptr::<CFdSet>::null().with_mut(|__s| {
                                        __s.zero();
                                        for (__fd, __bfd) in
                                            __w_fds.iter().zip(&__b[__rn..__rn + __wn])
                                        {
                                            if __ws.contains(*__bfd) {
                                                __s.set(*__fd);
                                            }
                                        }
                                    });
                                }
                                if !Ptr::<CFdSet>::null().is_null() {
                                    Ptr::<CFdSet>::null().with_mut(|__s| {
                                        __s.zero();
                                        for (__fd, __bfd) in __e_fds.iter().zip(&__b[__rn + __wn..])
                                        {
                                            if __es.contains(*__bfd) {
                                                __s.set(*__fd);
                                            }
                                        }
                                    });
                                }
                                match (__tmp_5.is_null(), __tv.as_ref()) {
                                    (false, Some(__t)) => __tmp_5.with_mut(|__dst| {
                                        __dst.tv_sec = __t.tv_sec() as i64;
                                        __dst.tv_usec = __t.tv_usec() as i64;
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
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((!(match ({ (*fds.borrow())[(0) as usize] },) {
            (mut __tmp_6,) => {
                if (rset.as_pointer()).with(|__s| __s.isset(__tmp_6)) {
                    1
                } else {
                    0
                }
            }
        } != 0) as i32)
            != 0)
    );
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(1) as usize], |__fd| {
            Ptr::<i8>::from_string_literal(b"x")
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
    match ({ (*fds.borrow())[(0) as usize] },) {
        (mut __tmp_7,) => {
            (rset.as_pointer()).with_mut(|__s| __s.set(__tmp_7));
        }
    };
    (*tv.borrow_mut()).tv_sec = (1 as ::libc::time_t);
    assert!(
        (((match (
            { ((*fds.borrow())[(0) as usize] + 1) },
            { (rset.as_pointer()) },
            { (tv.as_pointer()) },
        ) {
            (mut __tmp_8, mut __tmp_9, mut __tmp_10) => {
                {
                    let __r_fds: Vec<i32> = match __tmp_9.is_null() {
                        true => Vec::new(),
                        false => __tmp_9
                            .with(|__s| (0..__tmp_8).filter(|&__fd| __s.isset(__fd)).collect()),
                    };
                    let __w_fds: Vec<i32> = match Ptr::<CFdSet>::null().is_null() {
                        true => Vec::new(),
                        false => Ptr::<CFdSet>::null()
                            .with(|__s| (0..__tmp_8).filter(|&__fd| __s.isset(__fd)).collect()),
                    };
                    let __e_fds: Vec<i32> = match Ptr::<CFdSet>::null().is_null() {
                        true => Vec::new(),
                        false => Ptr::<CFdSet>::null()
                            .with(|__s| (0..__tmp_8).filter(|&__fd| __s.isset(__fd)).collect()),
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
                        let mut __tv = match __tmp_10.is_null() {
                            true => None,
                            false => Some(__tmp_10.with(|__t| {
                                nix::sys::time::TimeVal::new(__t.tv_sec as _, __t.tv_usec as _)
                            })),
                        };
                        match nix::sys::select::select(
                            __tmp_8,
                            match __tmp_9.is_null() {
                                true => None,
                                false => Some(&mut __rs),
                            },
                            match Ptr::<CFdSet>::null().is_null() {
                                true => None,
                                false => Some(&mut __ws),
                            },
                            match Ptr::<CFdSet>::null().is_null() {
                                true => None,
                                false => Some(&mut __es),
                            },
                            __tv.as_mut(),
                        ) {
                            Ok(__n) => {
                                if !__tmp_9.is_null() {
                                    __tmp_9.with_mut(|__s| {
                                        __s.zero();
                                        for (__fd, __bfd) in __r_fds.iter().zip(&__b[..__rn]) {
                                            if __rs.contains(*__bfd) {
                                                __s.set(*__fd);
                                            }
                                        }
                                    });
                                }
                                if !Ptr::<CFdSet>::null().is_null() {
                                    Ptr::<CFdSet>::null().with_mut(|__s| {
                                        __s.zero();
                                        for (__fd, __bfd) in
                                            __w_fds.iter().zip(&__b[__rn..__rn + __wn])
                                        {
                                            if __ws.contains(*__bfd) {
                                                __s.set(*__fd);
                                            }
                                        }
                                    });
                                }
                                if !Ptr::<CFdSet>::null().is_null() {
                                    Ptr::<CFdSet>::null().with_mut(|__s| {
                                        __s.zero();
                                        for (__fd, __bfd) in __e_fds.iter().zip(&__b[__rn + __wn..])
                                        {
                                            if __es.contains(*__bfd) {
                                                __s.set(*__fd);
                                            }
                                        }
                                    });
                                }
                                match (__tmp_10.is_null(), __tv.as_ref()) {
                                    (false, Some(__t)) => __tmp_10.with_mut(|__dst| {
                                        __dst.tv_sec = __t.tv_sec() as i64;
                                        __dst.tv_usec = __t.tv_usec() as i64;
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
                }
            }
        } == 1) as i32)
            != 0)
    );
    assert!(
        (match ({ (*fds.borrow())[(0) as usize] },) {
            (mut __tmp_11,) => {
                if (rset.as_pointer()).with(|__s| __s.isset(__tmp_11)) {
                    1
                } else {
                    0
                }
            }
        } != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
