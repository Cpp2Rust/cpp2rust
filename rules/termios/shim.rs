// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, DeepClone, Record, Value};
use std::cell::RefCell;
use std::mem::{offset_of, size_of};
use std::rc::Rc;

#[derive(DeepClone, Record, ByteRepr)]
#[byte_size(size_of::<::libc::termios>())]
pub struct Termios {
    #[offset(offset_of!(::libc::termios, c_iflag))]
    pub c_iflag: u32,
    #[offset(offset_of!(::libc::termios, c_oflag))]
    pub c_oflag: u32,
    #[offset(offset_of!(::libc::termios, c_cflag))]
    pub c_cflag: u32,
    #[offset(offset_of!(::libc::termios, c_lflag))]
    pub c_lflag: u32,
    #[cfg_attr(target_os = "linux", offset(offset_of!(::libc::termios, c_line)))]
    #[cfg_attr(target_os = "macos", offset(offset_of!(::libc::termios, c_cc)))]
    pub c_line: u8,
    #[offset(offset_of!(::libc::termios, c_cc))]
    #[byte_size(::libc::NCCS)]
    pub c_cc: Value<Box<[u8]>>,
    #[offset(offset_of!(::libc::termios, c_ispeed))]
    pub c_ispeed: u32,
    #[offset(offset_of!(::libc::termios, c_ospeed))]
    pub c_ospeed: u32,
}

impl Default for Termios {
    fn default() -> Self {
        Self {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: Rc::new(RefCell::new(vec![0u8; 32].into_boxed_slice())),
            c_ispeed: 0,
            c_ospeed: 0,
        }
    }
}

impl Termios {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(t: &::libc::termios) -> Self {
        let s = Self {
            c_iflag: t.c_iflag as u32,
            c_oflag: t.c_oflag as u32,
            c_cflag: t.c_cflag as u32,
            c_lflag: t.c_lflag as u32,
            #[cfg(target_os = "linux")]
            c_line: t.c_line,
            c_ispeed: t.c_ispeed as u32,
            c_ospeed: t.c_ospeed as u32,
            ..Self::default()
        };
        {
            let mut cc = s.c_cc.borrow_mut();
            let n = t.c_cc.len().min(cc.len());
            cc[..n].copy_from_slice(&t.c_cc[..n]);
        }
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::termios {
        ::libc::termios {
            c_iflag: self.c_iflag,
            c_oflag: self.c_oflag,
            c_cflag: self.c_cflag,
            c_lflag: self.c_lflag,
            c_line: self.c_line,
            c_cc: {
                let mut cc = [0u8; 32];
                let src = self.c_cc.borrow();
                let n = src.len().min(cc.len());
                cc[..n].copy_from_slice(&src[..n]);
                cc
            },
            c_ispeed: self.c_ispeed,
            c_ospeed: self.c_ospeed,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::termios {
        ::libc::termios {
            c_iflag: self.c_iflag as u64,
            c_oflag: self.c_oflag as u64,
            c_cflag: self.c_cflag as u64,
            c_lflag: self.c_lflag as u64,
            c_cc: {
                let mut cc = [0u8; 20];
                let src = self.c_cc.borrow();
                let n = src.len().min(cc.len());
                cc[..n].copy_from_slice(&src[..n]);
                cc
            },
            c_ispeed: self.c_ispeed as u64,
            c_ospeed: self.c_ospeed as u64,
        }
    }
}

#[derive(Clone, Default, Record, ByteRepr)]
#[byte_size(size_of::<::libc::winsize>())]
pub struct Winsize {
    #[offset(offset_of!(::libc::winsize, ws_row))]
    pub ws_row: u16,
    #[offset(offset_of!(::libc::winsize, ws_col))]
    pub ws_col: u16,
    #[offset(offset_of!(::libc::winsize, ws_xpixel))]
    pub ws_xpixel: u16,
    #[offset(offset_of!(::libc::winsize, ws_ypixel))]
    pub ws_ypixel: u16,
}
