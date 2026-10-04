// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Record};
use std::mem::{offset_of, size_of};

#[derive(Clone, Default, Record, ByteRepr)]
#[byte_size(size_of::<::libc::passwd>())]
pub struct Passwd {
    #[offset(offset_of!(::libc::passwd, pw_name))]
    pub pw_name: Ptr<i8>,
    #[offset(offset_of!(::libc::passwd, pw_passwd))]
    pub pw_passwd: Ptr<i8>,
    #[offset(offset_of!(::libc::passwd, pw_uid))]
    pub pw_uid: u32,
    #[offset(offset_of!(::libc::passwd, pw_gid))]
    pub pw_gid: u32,
    #[offset(offset_of!(::libc::passwd, pw_gecos))]
    pub pw_gecos: Ptr<i8>,
    #[offset(offset_of!(::libc::passwd, pw_dir))]
    pub pw_dir: Ptr<i8>,
    #[offset(offset_of!(::libc::passwd, pw_shell))]
    pub pw_shell: Ptr<i8>,
}

impl Passwd {
    pub fn from_user(u: &nix::unistd::User) -> Self {
        let mk = |s: &[u8]| -> Ptr<i8> { Ptr::alloc_c_str(s) };
        Self {
            pw_name: mk(u.name.as_bytes()),
            pw_passwd: mk(u.passwd.as_bytes()),
            pw_uid: u.uid.as_raw(),
            pw_gid: u.gid.as_raw(),
            pw_gecos: mk(u.gecos.as_bytes()),
            pw_dir: mk(u.dir.as_os_str().as_encoded_bytes()),
            pw_shell: mk(u.shell.as_os_str().as_encoded_bytes()),
        }
    }

    pub fn from_user_in(u: &nix::unistd::User, strings: &[Ptr<i8>]) -> Self {
        Self {
            pw_name: strings[0].clone(),
            pw_passwd: strings[1].clone(),
            pw_uid: u.uid.as_raw(),
            pw_gid: u.gid.as_raw(),
            pw_gecos: strings[2].clone(),
            pw_dir: strings[3].clone(),
            pw_shell: strings[4].clone(),
        }
    }
}

impl ByteRepr for ::libc::passwd {}
