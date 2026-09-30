extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    #[offset(0)]
    pub x1: i32,
}
impl ByteRepr for Level0_Level1_1_Level2_1_Level3_1 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
}
impl ByteRepr for Level0_Level1_1_Level2_1_Level3_2 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
        self.x2.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
            x2: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1_Level2_1 {
    #[offset(0)]
    pub x1: i32,
}
impl ByteRepr for Level0_Level1_1_Level2_1 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_1 {
    #[offset(0)]
    pub x1: i32,
}
impl ByteRepr for Level0_Level1_1 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Level0_Level1_2 {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
}
impl ByteRepr for Level0_Level1_2 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
        self.x2.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
            x2: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
pub struct Level0 {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<Level0_Level1_1> = Rc::new(RefCell::new(Level0_Level1_1 { x1: 0 }));
    let x2: Value<Level0_Level1_2> = Rc::new(RefCell::new(Level0_Level1_2 { x1: 1, x2: 2 }));
    let x3: Value<Level0_Level1_1_Level2_1> =
        Rc::new(RefCell::new(Level0_Level1_1_Level2_1 { x1: 3 }));
    let x4: Value<Level0_Level1_1_Level2_1_Level3_1> =
        Rc::new(RefCell::new(Level0_Level1_1_Level2_1_Level3_1 { x1: 4 }));
    let x5: Value<Level0_Level1_1_Level2_1_Level3_2> =
        Rc::new(RefCell::new(Level0_Level1_1_Level2_1_Level3_2 {
            x1: 5,
            x2: 6,
        }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
