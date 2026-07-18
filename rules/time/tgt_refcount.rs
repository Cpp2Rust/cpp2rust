// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Tm {
    Default::default()
}

fn t2() -> libcc2rs::Timeval {
    Default::default()
}

fn t3() -> libcc2rs::Timespec {
    Default::default()
}

fn f1(a0: Ptr<::libc::time_t>) -> ::libc::time_t {
    let __out = a0;
    match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
        Ok(__ts) => {
            let __s = __ts.tv_sec();
            if !__out.is_null() {
                __out.write(__s);
            }
            __s
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f2(a0: nix::time::ClockId, a1: Ptr<Timespec>) -> i32 {
    match nix::time::clock_gettime(a0) {
        Ok(__ts) => {
            a1.with_mut(|__t| {
                *__t.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                *__t.tv_nsec.borrow_mut() = __ts.tv_nsec() as i64;
            });
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f4(a0: Ptr<::libc::time_t>, a1: Ptr<Tm>) -> Ptr<Tm> {
    let __res = a1.clone();
    match jiff::Timestamp::from_second(a0.read()) {
        Ok(__ts) => {
            let __dt = __ts.to_zoned(jiff::tz::TimeZone::UTC);
            __res.with_mut(|__tm| *__tm = Tm::from_zoned(&__dt));
            __res
        }
        Err(_) => {
            libcc2rs::cpp2rust_errno().write(::libc::EOVERFLOW);
            Ptr::null()
        }
    }
}

fn f6(a0: Ptr<u8>, a1: usize, a2: Ptr<u8>, a3: Ptr<Tm>) -> usize {
    let __dt = a3.with(|__tm| __tm.to_civil());
    let __text = match __dt {
        Ok(__d) => {
            jiff::fmt::strtime::format(a2.to_rust_string().as_str(), __d).unwrap_or_default()
        }
        Err(_) => String::new(),
    };
    if __text.is_empty() || __text.len() + 1 > a1 {
        0
    } else {
        let mut __dst = a0.clone();
        for __b in __text.as_bytes() {
            __dst.write(*__b);
            __dst += 1;
        }
        __dst.write(0);
        __text.len()
    }
}

fn f7(a0: Ptr<u8>, a1: Ptr<Timeval>) -> i32 {
    let __times = a1;
    let __at = __times.with(|__tv| {
        nix::sys::time::TimeVal::new(
            *__tv.tv_sec.borrow() as ::libc::time_t,
            *__tv.tv_usec.borrow() as ::libc::suseconds_t,
        )
    });
    let __mt = __times.offset(1).with(|__tv| {
        nix::sys::time::TimeVal::new(
            *__tv.tv_sec.borrow() as ::libc::time_t,
            *__tv.tv_usec.borrow() as ::libc::suseconds_t,
        )
    });
    match nix::sys::stat::utimes(a0.to_rust_string().as_str(), &__at, &__mt) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

// a1 is ignored intentionally, see: https://man7.org/linux/man-pages/man2/gettimeofday.2.html
//
// "The use of the timezone structure is obsolete; the tz argument should normally be specified as NULL."
#[cfg(target_os = "linux")]
fn f8(a0: Ptr<Timeval>, a1: Ptr<::libc::timezone>) -> i32 {
    match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
        Ok(__ts) => {
            a0.with_mut(|__tv| {
                *__tv.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                *__tv.tv_usec.borrow_mut() = (__ts.tv_nsec() / 1000) as i64;
            });
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

// a1 is ignored intentionally, see: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/gettimeofday.2.html
//
// "Note: timezone is no longer used; this information is kept outside the kernel."
#[cfg(target_os = "macos")]
fn f8(a0: Ptr<Timeval>, a1: AnyPtr) -> i32 {
    match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
        Ok(__ts) => {
            a0.with_mut(|__tv| {
                *__tv.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                *__tv.tv_usec.borrow_mut() = (__ts.tv_nsec() / 1000) as i64;
            });
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

unsafe fn f9() -> nix::time::ClockId {
    nix::time::ClockId::CLOCK_REALTIME
}
