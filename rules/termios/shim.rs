// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Termios {
    pub c_iflag: Value<u32>,
    pub c_oflag: Value<u32>,
    pub c_cflag: Value<u32>,
    pub c_lflag: Value<u32>,
    pub c_line: Value<u8>,
    pub c_cc: Value<Box<[u8]>>,
    pub c_ispeed: Value<u32>,
    pub c_ospeed: Value<u32>,
}

impl Default for Termios {
    fn default() -> Self {
        Self {
            c_iflag: Rc::new(RefCell::new(0)),
            c_oflag: Rc::new(RefCell::new(0)),
            c_cflag: Rc::new(RefCell::new(0)),
            c_lflag: Rc::new(RefCell::new(0)),
            c_line: Rc::new(RefCell::new(0)),
            c_cc: Rc::new(RefCell::new(vec![0u8; 32].into_boxed_slice())),
            c_ispeed: Rc::new(RefCell::new(0)),
            c_ospeed: Rc::new(RefCell::new(0)),
        }
    }
}

impl Clone for Termios {
    fn clone(&self) -> Self {
        Self {
            c_iflag: Rc::new(RefCell::new(*self.c_iflag.borrow())),
            c_oflag: Rc::new(RefCell::new(*self.c_oflag.borrow())),
            c_cflag: Rc::new(RefCell::new(*self.c_cflag.borrow())),
            c_lflag: Rc::new(RefCell::new(*self.c_lflag.borrow())),
            c_line: Rc::new(RefCell::new(*self.c_line.borrow())),
            c_cc: Rc::new(RefCell::new(self.c_cc.borrow().clone())),
            c_ispeed: Rc::new(RefCell::new(*self.c_ispeed.borrow())),
            c_ospeed: Rc::new(RefCell::new(*self.c_ospeed.borrow())),
        }
    }
}

impl ByteRepr for Termios {}

impl Termios {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(t: &::libc::termios) -> Self {
        let s = Self::default();
        *s.c_iflag.borrow_mut() = t.c_iflag as u32;
        *s.c_oflag.borrow_mut() = t.c_oflag as u32;
        *s.c_cflag.borrow_mut() = t.c_cflag as u32;
        *s.c_lflag.borrow_mut() = t.c_lflag as u32;
        #[cfg(target_os = "linux")]
        {
            *s.c_line.borrow_mut() = t.c_line;
        }
        {
            let mut cc = s.c_cc.borrow_mut();
            let n = t.c_cc.len().min(cc.len());
            cc[..n].copy_from_slice(&t.c_cc[..n]);
        }
        *s.c_ispeed.borrow_mut() = t.c_ispeed as u32;
        *s.c_ospeed.borrow_mut() = t.c_ospeed as u32;
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::termios {
        ::libc::termios {
            c_iflag: *self.c_iflag.borrow(),
            c_oflag: *self.c_oflag.borrow(),
            c_cflag: *self.c_cflag.borrow(),
            c_lflag: *self.c_lflag.borrow(),
            c_line: *self.c_line.borrow(),
            c_cc: {
                let mut cc = [0u8; 32];
                let src = self.c_cc.borrow();
                let n = src.len().min(cc.len());
                cc[..n].copy_from_slice(&src[..n]);
                cc
            },
            c_ispeed: *self.c_ispeed.borrow(),
            c_ospeed: *self.c_ospeed.borrow(),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::termios {
        ::libc::termios {
            c_iflag: *self.c_iflag.borrow() as u64,
            c_oflag: *self.c_oflag.borrow() as u64,
            c_cflag: *self.c_cflag.borrow() as u64,
            c_lflag: *self.c_lflag.borrow() as u64,
            c_cc: {
                let mut cc = [0u8; 20];
                let src = self.c_cc.borrow();
                let n = src.len().min(cc.len());
                cc[..n].copy_from_slice(&src[..n]);
                cc
            },
            c_ispeed: *self.c_ispeed.borrow() as u64,
            c_ospeed: *self.c_ospeed.borrow() as u64,
        }
    }
}

#[derive(Default)]
pub struct Winsize {
    pub ws_row: Value<u16>,
    pub ws_col: Value<u16>,
    pub ws_xpixel: Value<u16>,
    pub ws_ypixel: Value<u16>,
}

impl Clone for Winsize {
    fn clone(&self) -> Self {
        Self {
            ws_row: Rc::new(RefCell::new(*self.ws_row.borrow())),
            ws_col: Rc::new(RefCell::new(*self.ws_col.borrow())),
            ws_xpixel: Rc::new(RefCell::new(*self.ws_xpixel.borrow())),
            ws_ypixel: Rc::new(RefCell::new(*self.ws_ypixel.borrow())),
        }
    }
}

impl ByteRepr for Winsize {}
