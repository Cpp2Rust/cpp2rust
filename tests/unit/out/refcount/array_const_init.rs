extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct S {
    #[offset(0)]
    pub head: i32,
    #[offset(4)]
    pub tail: Box<[i32]>,
    #[offset(16)]
    pub buf: Box<[u8]>,
}
impl Default for S {
    fn default() -> Self {
        S {
            head: 0_i32,
            tail: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
            buf: (0..4).map(|_| 0_u8).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        20
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.head.to_bytes(&mut buf[0..4]);
        self.tail.to_bytes(&mut buf[4..16]);
        self.buf.to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: <i32>::from_bytes(&buf[0..4]),
            tail: <Box<[i32]>>::from_bytes(&buf[4..16]),
            buf: <Box<[u8]>>::from_bytes(&buf[16..20]),
        }
    }
}
thread_local!(
    pub static s_0: Value<S> = Rc::new(RefCell::new(S {
        head: 5,
        tail: Box::new([0; 3]),
        buf: Box::new([0; 4]),
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((({ (*s_0.with(Value::clone).borrow()).head } == 5) as i32) != 0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 3) as i32) != 0) {
        assert!(
            ((((*s_0.with(Value::clone).borrow()).tail[(*i.borrow()) as usize] == 0) as i32) != 0)
        );
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
        assert!(
            (((((*s_0.with(Value::clone).borrow()).buf[(*i.borrow()) as usize] as i32) == 0)
                as i32)
                != 0)
        );
        (*i.borrow_mut()).postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
}
