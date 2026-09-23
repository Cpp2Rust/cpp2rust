// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Passwd {
    pub pw_name: Value<Ptr<u8>>,
    pub pw_passwd: Value<Ptr<u8>>,
    pub pw_uid: Value<u32>,
    pub pw_gid: Value<u32>,
    pub pw_gecos: Value<Ptr<u8>>,
    pub pw_dir: Value<Ptr<u8>>,
    pub pw_shell: Value<Ptr<u8>>,
}

impl Passwd {
    pub fn from_user(u: &nix::unistd::User) -> Self {
        let mk = |s: &[u8]| -> Value<Ptr<u8>> {
            let mut v = s.to_vec();
            v.push(0);
            Rc::new(RefCell::new(Ptr::alloc_array(v.into_boxed_slice())))
        };
        Self {
            pw_name: mk(u.name.as_bytes()),
            pw_passwd: mk(u.passwd.as_bytes()),
            pw_uid: Rc::new(RefCell::new(u.uid.as_raw())),
            pw_gid: Rc::new(RefCell::new(u.gid.as_raw())),
            pw_gecos: mk(u.gecos.as_bytes()),
            pw_dir: mk(u.dir.as_os_str().as_encoded_bytes()),
            pw_shell: mk(u.shell.as_os_str().as_encoded_bytes()),
        }
    }

    pub fn from_user_in(u: &nix::unistd::User, strings: &[Ptr<u8>]) -> Self {
        Self {
            pw_name: Rc::new(RefCell::new(strings[0].clone())),
            pw_passwd: Rc::new(RefCell::new(strings[1].clone())),
            pw_uid: Rc::new(RefCell::new(u.uid.as_raw())),
            pw_gid: Rc::new(RefCell::new(u.gid.as_raw())),
            pw_gecos: Rc::new(RefCell::new(strings[2].clone())),
            pw_dir: Rc::new(RefCell::new(strings[3].clone())),
            pw_shell: Rc::new(RefCell::new(strings[4].clone())),
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
