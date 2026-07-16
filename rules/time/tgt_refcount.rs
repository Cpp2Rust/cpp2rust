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

fn f2(a0: ::libc::clockid_t, a1: Ptr<Timespec>) -> i32 {
    match nix::time::clock_gettime(nix::time::ClockId::from_raw(a0)) {
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
    let __res = a1;
    match jiff::Timestamp::from_second(a0.read()) {
        Ok(__ts) => {
            let __dt = __ts.to_zoned(jiff::tz::TimeZone::UTC);
            __res.with_mut(|__tm| {
                *__tm.tm_sec.borrow_mut() = __dt.second() as i32;
                *__tm.tm_min.borrow_mut() = __dt.minute() as i32;
                *__tm.tm_hour.borrow_mut() = __dt.hour() as i32;
                *__tm.tm_mday.borrow_mut() = __dt.day() as i32;
                *__tm.tm_mon.borrow_mut() = __dt.month() as i32 - 1;
                *__tm.tm_year.borrow_mut() = __dt.year() as i32 - 1900;
                *__tm.tm_wday.borrow_mut() = __dt.weekday().to_sunday_zero_offset() as i32;
                *__tm.tm_yday.borrow_mut() = __dt.day_of_year() as i32 - 1;
                *__tm.tm_isdst.borrow_mut() = 0;
                *__tm.tm_gmtoff.borrow_mut() = 0;
                *__tm.tm_zone.borrow_mut() = Ptr::from_string_literal(b"GMT");
            });
            __res
        }
        Err(_) => {
            libcc2rs::cpp2rust_errno().write(::libc::EOVERFLOW);
            Ptr::null()
        }
    }
}

fn f6(a0: Ptr<u8>, a1: usize, a2: Ptr<u8>, a3: Ptr<Tm>) -> usize {
    let __dt = a3.with(|__tm| {
        jiff::civil::DateTime::new(
            (*__tm.tm_year.borrow() + 1900) as i16,
            (*__tm.tm_mon.borrow() + 1) as i8,
            *__tm.tm_mday.borrow() as i8,
            *__tm.tm_hour.borrow() as i8,
            *__tm.tm_min.borrow() as i8,
            *__tm.tm_sec.borrow() as i8,
            0,
        )
    });
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
