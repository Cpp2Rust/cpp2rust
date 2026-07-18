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
pub fn print_tm_1(t: i64) {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer()).clone();
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
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
        })
        .is_null())) as i32)
            != 0)
    );
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        (*(*tm.borrow()).tm_year.borrow()),
        (*(*tm.borrow()).tm_mon.borrow()),
        (*(*tm.borrow()).tm_mday.borrow()),
        (*(*tm.borrow()).tm_hour.borrow()),
        (*(*tm.borrow()).tm_min.borrow()),
        (*(*tm.borrow()).tm_sec.borrow()),
        (*(*tm.borrow()).tm_wday.borrow()),
        (*(*tm.borrow()).tm_yday.borrow()),
        (*(*tm.borrow()).tm_zone.borrow()),
        (*(*tm.borrow()).tm_gmtoff.borrow()),
        (*(*tm.borrow()).tm_isdst.borrow())
    );
}
pub fn test_gmtime_r_2() {
    ({ print_tm_1(0_i64) });
    ({ print_tm_1(1_i64) });
    ({ print_tm_1(86399_i64) });
    ({ print_tm_1(86400_i64) });
    ({ print_tm_1(951782400_i64) });
    ({ print_tm_1(951868799_i64) });
    ({ print_tm_1(1704067199_i64) });
    ({ print_tm_1(1704067200_i64) });
    ({ print_tm_1(1721126096_i64) });
    ({ print_tm_1(4102444800_i64) });
}
pub fn test_strftime_3() {
    let t: Value<i64> = Rc::new(RefCell::new(1721126096_i64));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer()).clone();
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
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
        })
        .is_null())) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
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
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
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
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
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
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
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
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
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
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_time_0() });
    ({ test_gmtime_r_2() });
    ({ test_strftime_3() });
    return 0;
}
