extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct POD {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
    #[offset(8)]
    pub x3: i32,
}
impl ByteRepr for POD {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
        self.x2.to_bytes(&mut buf[4..8]);
        self.x3.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
            x2: <i32>::from_bytes(&buf[4..8]),
            x3: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn PODIncrement_0(pod: Ptr<POD>) {
    {
        let _ptr = field!(pod, x1);
        _ptr.write(_ptr.read() + 1)
    };
    {
        let _ptr = field!(pod, x2);
        _ptr.write(_ptr.read() + 2)
    };
    {
        let _ptr = field!(pod, x3);
        _ptr.write(_ptr.read() + 3)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<POD> = Rc::new(RefCell::new(POD {
        x1: 10,
        x2: 11,
        x3: 12,
    }));
    let p2: Value<POD> = Rc::new(RefCell::new(POD {
        x1: { (*p1.borrow()).x1 },
        x2: { (*p1.borrow()).x2 },
        x3: { (*p1.borrow()).x3 },
    }));
    ({ PODIncrement_0(p2.as_pointer()) });
    assert!(((({ (*p2.borrow()).x1 } + { (*p2.borrow()).x2 }) + { (*p2.borrow()).x3 }) == 39));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
