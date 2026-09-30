// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Record};
use std::mem::{offset_of, size_of};

#[derive(Clone, Default, Record)]
pub struct Pollfd {
    #[offset(offset_of!(::libc::pollfd, fd))]
    pub fd: i32,
    #[offset(offset_of!(::libc::pollfd, events))]
    pub events: i16,
    #[offset(offset_of!(::libc::pollfd, revents))]
    pub revents: i16,
}

impl ByteRepr for Pollfd {
    fn byte_size() -> usize {
        size_of::<::libc::pollfd>()
    }
}

impl ByteRepr for ::libc::pollfd {}
