// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> CFdSet {
    Default::default()
}

fn f2(a0: i32, a1: Ptr<CFdSet>) {
    a1.with_mut(|__s| __s.set(a0));
}

fn f3(a0: i32, a1: Ptr<CFdSet>) {
    a1.with_mut(|__s| __s.clr(a0));
}

fn f4(a0: i32, a1: Ptr<CFdSet>) -> i32 {
    if a1.with(|__s| __s.isset(a0)) { 1 } else { 0 }
}

fn f5(a0: Ptr<CFdSet>) {
    a0.with_mut(|__s| __s.zero());
}

fn f1(a0: i32, a1: Ptr<CFdSet>, a2: Ptr<CFdSet>, a3: Ptr<CFdSet>, a4: Ptr<Timeval>) -> i32 {
    let __rp = a1.clone();
    let __wp = a2.clone();
    let __ep = a3.clone();
    let __tp = a4.clone();
    let __r_fds: Vec<i32> = match __rp.is_null() {
        true => Vec::new(),
        false => __rp.with(|__s| (0..a0).filter(|&__fd| __s.isset(__fd)).collect()),
    };
    let __w_fds: Vec<i32> = match __wp.is_null() {
        true => Vec::new(),
        false => __wp.with(|__s| (0..a0).filter(|&__fd| __s.isset(__fd)).collect()),
    };
    let __e_fds: Vec<i32> = match __ep.is_null() {
        true => Vec::new(),
        false => __ep.with(|__s| (0..a0).filter(|&__fd| __s.isset(__fd)).collect()),
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
                nix::sys::time::TimeVal::new(*__t.tv_sec.borrow() as _, *__t.tv_usec.borrow() as _)
            })),
        };
        match nix::sys::select::select(
            a0,
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
}
