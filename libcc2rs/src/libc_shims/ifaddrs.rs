// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::Sockaddr;
use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ifaddrs {
    pub ifa_next: Value<Ptr<Ifaddrs>>,
    pub ifa_name: Value<Ptr<u8>>,
    pub ifa_flags: Value<u32>,
    pub ifa_addr: Value<Ptr<Sockaddr>>,
    pub ifa_netmask: Value<Ptr<Sockaddr>>,
}

impl Default for Ifaddrs {
    fn default() -> Self {
        Self {
            ifa_next: Rc::new(RefCell::new(Ptr::null())),
            ifa_name: Rc::new(RefCell::new(Ptr::null())),
            ifa_flags: Rc::new(RefCell::new(0)),
            ifa_addr: Rc::new(RefCell::new(Ptr::null())),
            ifa_netmask: Rc::new(RefCell::new(Ptr::null())),
        }
    }
}

impl Clone for Ifaddrs {
    fn clone(&self) -> Self {
        Self {
            ifa_next: Rc::new(RefCell::new(self.ifa_next.borrow().clone())),
            ifa_name: Rc::new(RefCell::new(self.ifa_name.borrow().clone())),
            ifa_flags: Rc::new(RefCell::new(*self.ifa_flags.borrow())),
            ifa_addr: Rc::new(RefCell::new(self.ifa_addr.borrow().clone())),
            ifa_netmask: Rc::new(RefCell::new(self.ifa_netmask.borrow().clone())),
        }
    }
}

impl ByteRepr for Ifaddrs {}

impl ByteRepr for ::libc::ifaddrs {}
