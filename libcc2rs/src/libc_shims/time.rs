// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Tm {
    pub tm_sec: Value<i32>,
    pub tm_min: Value<i32>,
    pub tm_hour: Value<i32>,
    pub tm_mday: Value<i32>,
    pub tm_mon: Value<i32>,
    pub tm_year: Value<i32>,
    pub tm_wday: Value<i32>,
    pub tm_yday: Value<i32>,
    pub tm_isdst: Value<i32>,
    pub tm_gmtoff: Value<i64>,
    pub tm_zone: Value<Ptr<u8>>,
}

impl Clone for Tm {
    fn clone(&self) -> Self {
        Self {
            tm_sec: Rc::new(RefCell::new(*self.tm_sec.borrow())),
            tm_min: Rc::new(RefCell::new(*self.tm_min.borrow())),
            tm_hour: Rc::new(RefCell::new(*self.tm_hour.borrow())),
            tm_mday: Rc::new(RefCell::new(*self.tm_mday.borrow())),
            tm_mon: Rc::new(RefCell::new(*self.tm_mon.borrow())),
            tm_year: Rc::new(RefCell::new(*self.tm_year.borrow())),
            tm_wday: Rc::new(RefCell::new(*self.tm_wday.borrow())),
            tm_yday: Rc::new(RefCell::new(*self.tm_yday.borrow())),
            tm_isdst: Rc::new(RefCell::new(*self.tm_isdst.borrow())),
            tm_gmtoff: Rc::new(RefCell::new(*self.tm_gmtoff.borrow())),
            tm_zone: Rc::new(RefCell::new(self.tm_zone.borrow().clone())),
        }
    }
}

impl ByteRepr for Tm {}

#[derive(Default)]
pub struct Timeval {
    pub tv_sec: Value<i64>,
    pub tv_usec: Value<i64>,
}

impl Clone for Timeval {
    fn clone(&self) -> Self {
        Self {
            tv_sec: Rc::new(RefCell::new(*self.tv_sec.borrow())),
            tv_usec: Rc::new(RefCell::new(*self.tv_usec.borrow())),
        }
    }
}

impl ByteRepr for Timeval {}

#[derive(Default)]
pub struct Timespec {
    pub tv_sec: Value<i64>,
    pub tv_nsec: Value<i64>,
}

impl Clone for Timespec {
    fn clone(&self) -> Self {
        Self {
            tv_sec: Rc::new(RefCell::new(*self.tv_sec.borrow())),
            tv_nsec: Rc::new(RefCell::new(*self.tv_nsec.borrow())),
        }
    }
}

impl ByteRepr for Timespec {}

impl ByteRepr for ::libc::tm {}
impl ByteRepr for ::libc::timeval {}
impl ByteRepr for ::libc::timespec {}
