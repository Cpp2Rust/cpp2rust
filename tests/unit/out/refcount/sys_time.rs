extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_time_0() {
    let t1: Value<i64> = Rc::new(RefCell::new({
        let __out = Ptr::<i64>::null();
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
    }));
    let t2: Value<i64> = Rc::new(RefCell::new(0_i64));
    let t3: Value<i64> = Rc::new(RefCell::new({
        let __out = (t2.as_pointer());
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
    }));
    assert!(((((*t1.borrow()) > 1500000000_i64) as i32) != 0));
    assert!(((((*t2.borrow()) == (*t3.borrow())) as i32) != 0));
    assert!(((((*t3.borrow()) >= (*t1.borrow())) as i32) != 0));
}
pub fn test_clock_gettime_1() {
    let ts: Value<libcc2rs::Timespec> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match nix::time::clock_gettime(nix::time::ClockId::from_raw(0)) {
            Ok(__ts) => {
                (ts.as_pointer()).with_mut(|__t| {
                    *__t.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                    *__t.tv_nsec.borrow_mut() = __ts.tv_nsec() as i64;
                });
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*ts.borrow()).tv_sec.borrow()) > 1500000000_i64) as i32) != 0));
    assert!(
        (((((((*(*ts.borrow()).tv_nsec.borrow()) >= 0_i64) as i32) != 0)
            && ((((*(*ts.borrow()).tv_nsec.borrow()) < 1000000000_i64) as i32) != 0))
            as i32)
            != 0)
    );
}
pub fn print_tm_2(t: i64) {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer());
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
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
        })
        .is_null())) as i32)
            != 0)
    );
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={}",
        (*(*tm.borrow()).tm_year.borrow()),
        (*(*tm.borrow()).tm_mon.borrow()),
        (*(*tm.borrow()).tm_mday.borrow()),
        (*(*tm.borrow()).tm_hour.borrow()),
        (*(*tm.borrow()).tm_min.borrow()),
        (*(*tm.borrow()).tm_sec.borrow()),
        (*(*tm.borrow()).tm_wday.borrow()),
        (*(*tm.borrow()).tm_yday.borrow())
    );
}
pub fn test_gmtime_r_3() {
    ({ print_tm_2(0_i64) });
    ({ print_tm_2(1_i64) });
    ({ print_tm_2(86399_i64) });
    ({ print_tm_2(86400_i64) });
    ({ print_tm_2(951782400_i64) });
    ({ print_tm_2(951868799_i64) });
    ({ print_tm_2(1704067199_i64) });
    ({ print_tm_2(1704067200_i64) });
    ({ print_tm_2(1721126096_i64) });
    ({ print_tm_2(4102444800_i64) });
}
pub fn test_strftime_4() {
    let t: Value<i64> = Rc::new(RefCell::new(1721126096_i64));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer());
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
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
        })
        .is_null())) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| {
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
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%Y-%m-%d %H:%M:%S")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                let mut __dst = (buf.as_pointer() as Ptr<u8>).clone();
                for __b in __text.as_bytes() {
                    __dst.write(*__b);
                    __dst += 1;
                }
                __dst.write(0);
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| {
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
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%a, %d %b %Y %T")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                let mut __dst = (buf.as_pointer() as Ptr<u8>).clone();
                for __b in __text.as_bytes() {
                    __dst.write(*__b);
                    __dst += 1;
                }
                __dst.write(0);
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| {
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
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"day %j 100%%")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                let mut __dst = (buf.as_pointer() as Ptr<u8>).clone();
                for __b in __text.as_bytes() {
                    __dst.write(*__b);
                    __dst += 1;
                }
                __dst.write(0);
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| {
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
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%e").to_rust_string().as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                let mut __dst = (buf.as_pointer() as Ptr<u8>).clone();
                for __b in __text.as_bytes() {
                    __dst.write(*__b);
                    __dst += 1;
                }
                __dst.write(0);
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    let small: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| {
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
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%Y-%m-%d")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 4]>() {
                0
            } else {
                let mut __dst = (small.as_pointer() as Ptr<u8>).clone();
                for __b in __text.as_bytes() {
                    __dst.write(*__b);
                    __dst += 1;
                }
                __dst.write(0);
                __text.len()
            }
        } == 0_usize) as i32)
            != 0)
    );
}
pub fn test_gettimeofday_5() {
    let tv: Value<libcc2rs::Timeval> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
            Ok(__ts) => {
                (tv.as_pointer()).with_mut(|__tv| {
                    *__tv.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                    *__tv.tv_usec.borrow_mut() = (__ts.tv_nsec() / 1000) as i64;
                });
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*tv.borrow()).tv_sec.borrow()) > 1500000000_i64) as i32) != 0));
    assert!(
        (((((((*(*tv.borrow()).tv_usec.borrow()) >= 0_i64) as i32) != 0)
            && ((((*(*tv.borrow()).tv_usec.borrow()) < 1000000_i64) as i32) != 0))
            as i32)
            != 0)
    );
}
pub fn test_utimes_6() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_utimes_test.tmp",
    )));
    let fp: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
        match Ptr::from_string_literal(b"wb").to_rust_string() {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open((*path.borrow()).to_rust_string())
                .ok()
                .map_or(Ptr::null(), |f| Ptr::alloc(f)),
            v if v == "wb" => std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open((*path.borrow()).to_rust_string())
                .ok()
                .map_or(Ptr::null(), |f| Ptr::alloc(f)),
            _ => panic!("unsupported mode"),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            (*fp.borrow()).delete();
            0
        } == 0) as i32)
            != 0)
    );
    let times: Value<Box<[libcc2rs::Timeval]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| Default::default())
            .collect::<Box<[libcc2rs::Timeval]>>(),
    ));
    (*(*times.borrow())[(0) as usize].tv_sec.borrow_mut()) = 1000000000_i64;
    (*(*times.borrow())[(0) as usize].tv_usec.borrow_mut()) = 0_i64;
    (*(*times.borrow())[(1) as usize].tv_sec.borrow_mut()) = 1000000001_i64;
    (*(*times.borrow())[(1) as usize].tv_usec.borrow_mut()) = 0_i64;
    assert!(
        ((({
            let __times = (times.as_pointer() as Ptr<libcc2rs::Timeval>);
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
            match nix::sys::stat::utimes((*path.borrow()).to_rust_string().as_str(), &__at, &__mt) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __times = (times.as_pointer() as Ptr<libcc2rs::Timeval>);
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
            match nix::sys::stat::utimes(
                Ptr::from_string_literal(b"/tmp/cpp2rust_utimes_test_missing.tmp")
                    .to_rust_string()
                    .as_str(),
                &__at,
                &__mt,
            ) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == -1_i32) as i32)
            != 0)
    );
    assert!(
        (((match nix::unistd::unlink((*path.borrow()).to_rust_string().as_str()) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_time_0() });
    ({ test_clock_gettime_1() });
    ({ test_gmtime_r_3() });
    ({ test_strftime_4() });
    ({ test_gettimeofday_5() });
    ({ test_utimes_6() });
    return 0;
}
