// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Passwd {
    pub pw_name: Value<Ptr<u8>>,
    pub pw_passwd: Value<Ptr<u8>>,
    pub pw_uid: Value<u32>,
    pub pw_gid: Value<u32>,
    pub pw_gecos: Value<Ptr<u8>>,
    pub pw_dir: Value<Ptr<u8>>,
    pub pw_shell: Value<Ptr<u8>>,
}

impl Default for Passwd {
    fn default() -> Self {
        Self {
            pw_name: Rc::new(RefCell::new(Ptr::null())),
            pw_passwd: Rc::new(RefCell::new(Ptr::null())),
            pw_uid: Rc::new(RefCell::new(0)),
            pw_gid: Rc::new(RefCell::new(0)),
            pw_gecos: Rc::new(RefCell::new(Ptr::null())),
            pw_dir: Rc::new(RefCell::new(Ptr::null())),
            pw_shell: Rc::new(RefCell::new(Ptr::null())),
        }
    }
}

impl Clone for Passwd {
    fn clone(&self) -> Self {
        Self {
            pw_name: Rc::new(RefCell::new(self.pw_name.borrow().clone())),
            pw_passwd: Rc::new(RefCell::new(self.pw_passwd.borrow().clone())),
            pw_uid: Rc::new(RefCell::new(*self.pw_uid.borrow())),
            pw_gid: Rc::new(RefCell::new(*self.pw_gid.borrow())),
            pw_gecos: Rc::new(RefCell::new(self.pw_gecos.borrow().clone())),
            pw_dir: Rc::new(RefCell::new(self.pw_dir.borrow().clone())),
            pw_shell: Rc::new(RefCell::new(self.pw_shell.borrow().clone())),
        }
    }
}

impl ByteRepr for Passwd {}

impl ByteRepr for ::libc::passwd {}
