// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Record, Value};
use std::cell::{Cell, RefCell};
use std::mem::{offset_of, size_of};
use std::rc::Rc;

#[derive(Record)]
pub struct Dirent {
    #[offset(offset_of!(::libc::dirent, d_ino))]
    pub d_ino: u64,
    #[cfg_attr(target_os = "linux", offset(offset_of!(::libc::dirent, d_off)))]
    #[cfg_attr(target_os = "macos", offset(offset_of!(::libc::dirent, d_seekoff)))]
    pub d_off: i64,
    #[offset(offset_of!(::libc::dirent, d_reclen))]
    pub d_reclen: u16,
    #[offset(offset_of!(::libc::dirent, d_type))]
    pub d_type: u8,
    #[offset(offset_of!(::libc::dirent, d_name))]
    pub d_name: Value<Box<[u8]>>,
}

impl Default for Dirent {
    fn default() -> Self {
        Self {
            d_ino: 0,
            d_off: 0,
            d_reclen: 0,
            d_type: 0,
            d_name: Rc::new(RefCell::new(vec![0u8; 256].into_boxed_slice())),
        }
    }
}

impl Dirent {
    pub fn from_entry(ino: u64, name: &[u8], d_type: u8) -> Self {
        let de = Dirent {
            d_ino: ino,
            d_type,
            ..Dirent::default()
        };
        {
            let mut nm = de.d_name.borrow_mut();
            let n = name.len().min(nm.len() - 1);
            nm[..n].copy_from_slice(&name[..n]);
            nm[n] = 0;
        }
        de
    }
}

impl Clone for Dirent {
    fn clone(&self) -> Self {
        Self {
            d_name: Rc::new(RefCell::new(self.d_name.borrow().clone())),
            ..*self
        }
    }
}

impl ByteRepr for Dirent {
    fn byte_size() -> usize {
        size_of::<::libc::dirent>()
    }
}

pub struct CDir {
    pub entries: Vec<(u64, Vec<u8>, u8)>,
    pub pos: Cell<usize>,
}

impl CDir {
    pub fn from_dir(dir: nix::dir::Dir) -> Self {
        let mut entries: Vec<(u64, Vec<u8>, u8)> = Vec::new();
        for ent in dir.into_iter().flatten() {
            let ty = match ent.file_type() {
                Some(nix::dir::Type::Fifo) => ::libc::DT_FIFO,
                Some(nix::dir::Type::CharacterDevice) => ::libc::DT_CHR,
                Some(nix::dir::Type::Directory) => ::libc::DT_DIR,
                Some(nix::dir::Type::BlockDevice) => ::libc::DT_BLK,
                Some(nix::dir::Type::File) => ::libc::DT_REG,
                Some(nix::dir::Type::Symlink) => ::libc::DT_LNK,
                Some(nix::dir::Type::Socket) => ::libc::DT_SOCK,
                None => ::libc::DT_UNKNOWN,
            };
            entries.push((ent.ino(), ent.file_name().to_bytes().to_vec(), ty));
        }
        Self {
            entries,
            pos: Cell::new(0),
        }
    }
}

impl ByteRepr for CDir {}

impl ByteRepr for ::libc::dirent {}
