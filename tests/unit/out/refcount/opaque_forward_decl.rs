extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct container {
    #[offset(0)]
    pub p: Ptr<opaque>,
    #[offset(8)]
    pub x: i32,
}
impl ByteRepr for container {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.p.to_bytes(&mut buf[0..8]);
        self.x.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: <Ptr<opaque>>::from_bytes(&buf[0..8]),
            x: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<container> = Rc::new(RefCell::new(container {
        p: Ptr::<opaque>::null(),
        x: 42,
    }));
    &({ (*c.borrow()).p.clone() });
    return ({ (*c.borrow()).x } - 42);
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
pub struct opaque;
pub fn __cpp2rust_init_globals() {}
