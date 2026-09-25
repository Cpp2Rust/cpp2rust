// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Stat {
    pub st_dev: Value<u64>,
    pub st_ino: Value<u64>,
    pub st_nlink: Value<u64>,
    pub st_mode: Value<u32>,
    pub st_uid: Value<u32>,
    pub st_gid: Value<u32>,
    pub st_rdev: Value<u64>,
    pub st_size: Value<i64>,
    pub st_blksize: Value<i64>,
    pub st_blocks: Value<i64>,
    pub st_atime: Value<i64>,
    pub st_mtime: Value<i64>,
    pub st_ctime: Value<i64>,
}

impl Stat {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(s: &::libc::stat) -> Self {
        Self {
            st_dev: Rc::new(RefCell::new(s.st_dev as u64)),
            st_ino: Rc::new(RefCell::new(s.st_ino as u64)),
            st_nlink: Rc::new(RefCell::new(s.st_nlink as u64)),
            st_mode: Rc::new(RefCell::new(s.st_mode as u32)),
            st_uid: Rc::new(RefCell::new(s.st_uid)),
            st_gid: Rc::new(RefCell::new(s.st_gid)),
            st_rdev: Rc::new(RefCell::new(s.st_rdev as u64)),
            st_size: Rc::new(RefCell::new(s.st_size as i64)),
            st_blksize: Rc::new(RefCell::new(s.st_blksize as i64)),
            st_blocks: Rc::new(RefCell::new(s.st_blocks as i64)),
            st_atime: Rc::new(RefCell::new(s.st_atime as i64)),
            st_mtime: Rc::new(RefCell::new(s.st_mtime as i64)),
            st_ctime: Rc::new(RefCell::new(s.st_ctime as i64)),
        }
    }
}

impl Clone for Stat {
    fn clone(&self) -> Self {
        Self {
            st_dev: Rc::new(RefCell::new(*self.st_dev.borrow())),
            st_ino: Rc::new(RefCell::new(*self.st_ino.borrow())),
            st_nlink: Rc::new(RefCell::new(*self.st_nlink.borrow())),
            st_mode: Rc::new(RefCell::new(*self.st_mode.borrow())),
            st_uid: Rc::new(RefCell::new(*self.st_uid.borrow())),
            st_gid: Rc::new(RefCell::new(*self.st_gid.borrow())),
            st_rdev: Rc::new(RefCell::new(*self.st_rdev.borrow())),
            st_size: Rc::new(RefCell::new(*self.st_size.borrow())),
            st_blksize: Rc::new(RefCell::new(*self.st_blksize.borrow())),
            st_blocks: Rc::new(RefCell::new(*self.st_blocks.borrow())),
            st_atime: Rc::new(RefCell::new(*self.st_atime.borrow())),
            st_mtime: Rc::new(RefCell::new(*self.st_mtime.borrow())),
            st_ctime: Rc::new(RefCell::new(*self.st_ctime.borrow())),
        }
    }
}

impl ByteRepr for Stat {}

impl ByteRepr for ::libc::stat {}
