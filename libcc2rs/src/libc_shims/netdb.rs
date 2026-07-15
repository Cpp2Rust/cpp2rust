// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::Sockaddr;
use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Addrinfo {
    pub ai_flags: Value<i32>,
    pub ai_family: Value<i32>,
    pub ai_socktype: Value<i32>,
    pub ai_protocol: Value<i32>,
    pub ai_addrlen: Value<u32>,
    pub ai_addr: Value<Ptr<Sockaddr>>,
    pub ai_canonname: Value<Ptr<u8>>,
    pub ai_next: Value<Ptr<Addrinfo>>,
}

impl Default for Addrinfo {
    fn default() -> Self {
        Self {
            ai_flags: Rc::new(RefCell::new(0)),
            ai_family: Rc::new(RefCell::new(0)),
            ai_socktype: Rc::new(RefCell::new(0)),
            ai_protocol: Rc::new(RefCell::new(0)),
            ai_addrlen: Rc::new(RefCell::new(0)),
            ai_addr: Rc::new(RefCell::new(Ptr::null())),
            ai_canonname: Rc::new(RefCell::new(Ptr::null())),
            ai_next: Rc::new(RefCell::new(Ptr::null())),
        }
    }
}

impl Clone for Addrinfo {
    fn clone(&self) -> Self {
        Self {
            ai_flags: Rc::new(RefCell::new(*self.ai_flags.borrow())),
            ai_family: Rc::new(RefCell::new(*self.ai_family.borrow())),
            ai_socktype: Rc::new(RefCell::new(*self.ai_socktype.borrow())),
            ai_protocol: Rc::new(RefCell::new(*self.ai_protocol.borrow())),
            ai_addrlen: Rc::new(RefCell::new(*self.ai_addrlen.borrow())),
            ai_addr: Rc::new(RefCell::new(self.ai_addr.borrow().clone())),
            ai_canonname: Rc::new(RefCell::new(self.ai_canonname.borrow().clone())),
            ai_next: Rc::new(RefCell::new(self.ai_next.borrow().clone())),
        }
    }
}

impl ByteRepr for Addrinfo {}

impl ByteRepr for ::libc::addrinfo {}
