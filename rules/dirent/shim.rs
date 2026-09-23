// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Value};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct Dirent {
    pub d_ino: Value<u64>,
    pub d_off: Value<i64>,
    pub d_reclen: Value<u16>,
    pub d_type: Value<u8>,
    pub d_name: Value<Box<[u8]>>,
}

impl Default for Dirent {
    fn default() -> Self {
        Self {
            d_ino: Rc::new(RefCell::new(0)),
            d_off: Rc::new(RefCell::new(0)),
            d_reclen: Rc::new(RefCell::new(0)),
            d_type: Rc::new(RefCell::new(0)),
            d_name: Rc::new(RefCell::new(vec![0u8; 256].into_boxed_slice())),
        }
    }
}

impl Dirent {
    pub fn from_entry(ino: u64, name: &[u8], d_type: u8) -> Self {
        let de = Dirent::default();
        *de.d_ino.borrow_mut() = ino;
        *de.d_type.borrow_mut() = d_type;
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
            d_ino: Rc::new(RefCell::new(*self.d_ino.borrow())),
            d_off: Rc::new(RefCell::new(*self.d_off.borrow())),
            d_reclen: Rc::new(RefCell::new(*self.d_reclen.borrow())),
            d_type: Rc::new(RefCell::new(*self.d_type.borrow())),
            d_name: Rc::new(RefCell::new(self.d_name.borrow().clone())),
        }
    }
}

impl ByteRepr for Dirent {}

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
