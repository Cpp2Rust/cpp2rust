// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Record};
use std::mem::{offset_of, size_of};

#[derive(Clone, Default, Record)]
pub struct Tm {
    #[offset(offset_of!(::libc::tm, tm_sec))]
    pub tm_sec: i32,
    #[offset(offset_of!(::libc::tm, tm_min))]
    pub tm_min: i32,
    #[offset(offset_of!(::libc::tm, tm_hour))]
    pub tm_hour: i32,
    #[offset(offset_of!(::libc::tm, tm_mday))]
    pub tm_mday: i32,
    #[offset(offset_of!(::libc::tm, tm_mon))]
    pub tm_mon: i32,
    #[offset(offset_of!(::libc::tm, tm_year))]
    pub tm_year: i32,
    #[offset(offset_of!(::libc::tm, tm_wday))]
    pub tm_wday: i32,
    #[offset(offset_of!(::libc::tm, tm_yday))]
    pub tm_yday: i32,
    #[offset(offset_of!(::libc::tm, tm_isdst))]
    pub tm_isdst: i32,
    #[offset(offset_of!(::libc::tm, tm_gmtoff))]
    pub tm_gmtoff: i64,
    #[offset(offset_of!(::libc::tm, tm_zone))]
    pub tm_zone: Ptr<u8>,
}

impl Tm {
    pub fn from_zoned(dt: &jiff::Zoned) -> Self {
        #[cfg(target_os = "linux")]
        let zone: &'static [u8] = b"GMT";
        #[cfg(target_os = "macos")]
        let zone: &'static [u8] = b"UTC";
        Tm {
            tm_sec: dt.second() as i32,
            tm_min: dt.minute() as i32,
            tm_hour: dt.hour() as i32,
            tm_mday: dt.day() as i32,
            tm_mon: dt.month() as i32 - 1,
            tm_year: dt.year() as i32 - 1900,
            tm_wday: dt.weekday().to_sunday_zero_offset() as i32,
            tm_yday: dt.day_of_year() as i32 - 1,
            tm_isdst: 0,
            tm_gmtoff: dt.offset().seconds() as i64,
            tm_zone: Ptr::<u8>::from_string_literal(zone),
        }
    }

    pub fn to_civil(&self) -> Result<jiff::civil::DateTime, jiff::Error> {
        jiff::civil::DateTime::new(
            (self.tm_year + 1900) as i16,
            (self.tm_mon + 1) as i8,
            self.tm_mday as i8,
            self.tm_hour as i8,
            self.tm_min as i8,
            self.tm_sec as i8,
            0,
        )
    }
}

impl ByteRepr for Tm {
    fn byte_size() -> usize {
        size_of::<::libc::tm>()
    }
}

#[derive(Clone, Default, Record)]
pub struct Timeval {
    #[offset(offset_of!(::libc::timeval, tv_sec))]
    pub tv_sec: i64,
    #[offset(offset_of!(::libc::timeval, tv_usec))]
    pub tv_usec: i64,
}

impl ByteRepr for Timeval {
    fn byte_size() -> usize {
        16
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.tv_sec.to_bytes(&mut buf[0..8]);
        self.tv_usec.to_bytes(&mut buf[8..16]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tv_sec: i64::from_bytes(&buf[0..8]),
            tv_usec: i64::from_bytes(&buf[8..16]),
        }
    }
}

#[derive(Clone, Default, Record)]
pub struct Timespec {
    #[offset(offset_of!(::libc::timespec, tv_sec))]
    pub tv_sec: i64,
    #[offset(offset_of!(::libc::timespec, tv_nsec))]
    pub tv_nsec: i64,
}

impl ByteRepr for Timespec {
    fn byte_size() -> usize {
        size_of::<::libc::timespec>()
    }
}

impl ByteRepr for ::libc::tm {}
impl ByteRepr for ::libc::timeval {}
impl ByteRepr for ::libc::timespec {}
