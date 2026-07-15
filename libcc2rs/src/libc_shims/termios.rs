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

pub struct Winsize {
    pub ws_row: Value<u16>,
    pub ws_col: Value<u16>,
    pub ws_xpixel: Value<u16>,
    pub ws_ypixel: Value<u16>,
}

impl Default for Winsize {
    fn default() -> Self {
        Self {
            ws_row: Rc::new(RefCell::new(0)),
            ws_col: Rc::new(RefCell::new(0)),
            ws_xpixel: Rc::new(RefCell::new(0)),
            ws_ypixel: Rc::new(RefCell::new(0)),
        }
    }
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
