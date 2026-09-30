// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Record};
use std::mem::{offset_of, size_of};

#[derive(Clone, Default, Record)]
// The fields have the sizes of those of libc::stat only in x86-64 Linux.
#[cfg_attr(
    all(target_os = "linux", target_arch = "x86_64"),
    derive(ByteRepr),
    byte_size(size_of::<::libc::stat>())
)]
pub struct Stat {
    #[offset(offset_of!(::libc::stat, st_dev))]
    pub st_dev: u64,
    #[offset(offset_of!(::libc::stat, st_ino))]
    pub st_ino: u64,
    #[offset(offset_of!(::libc::stat, st_nlink))]
    pub st_nlink: u64,
    #[offset(offset_of!(::libc::stat, st_mode))]
    pub st_mode: u32,
    #[offset(offset_of!(::libc::stat, st_uid))]
    pub st_uid: u32,
    #[offset(offset_of!(::libc::stat, st_gid))]
    pub st_gid: u32,
    #[offset(offset_of!(::libc::stat, st_rdev))]
    pub st_rdev: u64,
    #[offset(offset_of!(::libc::stat, st_size))]
    pub st_size: i64,
    #[offset(offset_of!(::libc::stat, st_blksize))]
    pub st_blksize: i64,
    #[offset(offset_of!(::libc::stat, st_blocks))]
    pub st_blocks: i64,
    #[offset(offset_of!(::libc::stat, st_atime))]
    pub st_atime: i64,
    #[offset(offset_of!(::libc::stat, st_mtime))]
    pub st_mtime: i64,
    #[offset(offset_of!(::libc::stat, st_ctime))]
    pub st_ctime: i64,
}

impl Stat {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(s: &::libc::stat) -> Self {
        Self {
            st_dev: s.st_dev as u64,
            st_ino: s.st_ino as u64,
            st_nlink: s.st_nlink as u64,
            st_mode: s.st_mode as u32,
            st_uid: s.st_uid,
            st_gid: s.st_gid,
            st_rdev: s.st_rdev as u64,
            st_size: s.st_size as i64,
            st_blksize: s.st_blksize as i64,
            st_blocks: s.st_blocks as i64,
            st_atime: s.st_atime as i64,
            st_mtime: s.st_mtime as i64,
            st_ctime: s.st_ctime as i64,
        }
    }
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
impl ByteRepr for Stat {
    fn byte_size() -> usize {
        size_of::<::libc::stat>()
    }
}

impl ByteRepr for ::libc::stat {}
