// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr, Record, Sockaddr};
use std::mem::{offset_of, size_of};

#[derive(Clone, Default, Record)]
pub struct Addrinfo {
    #[offset(offset_of!(::libc::addrinfo, ai_flags))]
    pub ai_flags: i32,
    #[offset(offset_of!(::libc::addrinfo, ai_family))]
    pub ai_family: i32,
    #[offset(offset_of!(::libc::addrinfo, ai_socktype))]
    pub ai_socktype: i32,
    #[offset(offset_of!(::libc::addrinfo, ai_protocol))]
    pub ai_protocol: i32,
    #[offset(offset_of!(::libc::addrinfo, ai_addrlen))]
    pub ai_addrlen: u32,
    #[offset(offset_of!(::libc::addrinfo, ai_addr))]
    pub ai_addr: Ptr<Sockaddr>,
    #[offset(offset_of!(::libc::addrinfo, ai_canonname))]
    pub ai_canonname: Ptr<u8>,
    #[offset(offset_of!(::libc::addrinfo, ai_next))]
    pub ai_next: Ptr<Addrinfo>,
}

impl ByteRepr for Addrinfo {
    fn byte_size() -> usize {
        size_of::<::libc::addrinfo>()
    }
}

impl ByteRepr for ::libc::addrinfo {}
