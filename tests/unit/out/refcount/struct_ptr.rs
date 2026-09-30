extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct XX {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for XX {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let obj: Value<XX> = Rc::new(RefCell::new(<XX>::default()));
    let ptr: Value<Ptr<XX>> = Rc::new(RefCell::new((obj.as_pointer())));
    (*ptr.borrow()).with_mut(|__s: &mut XX| __s.x = 2);
    let c: Value<bool> = Rc::new(RefCell::new(false));
    let r: Value<i32> = Rc::new(RefCell::new(if (*c.borrow()) {
        { (*obj.borrow()).x }
    } else {
        (*ptr.borrow()).with(|__s: &XX| __s.x)
    }));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((field_ptr!(obj, x))));
    assert!(
        ({
            let _lhs = ((*p.borrow()).read());
            _lhs + (*r.borrow())
        } == 4)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
