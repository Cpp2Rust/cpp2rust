extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct S {
    pub head: Value<i32>,
    pub tail: Value<Box<[i32]>>,
    pub buf: Value<Box<[u8]>>,
}
impl Default for S {
    fn default() -> Self {
        S {
            head: <Value<i32>>::default(),
            tail: Rc::new(RefCell::new(
                (0..3).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            buf: Rc::new(RefCell::new(
                (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        20
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.head.borrow()).to_bytes(&mut buf[0..4]);
        (*self.tail.borrow()).to_bytes(&mut buf[4..16]);
        (*self.buf.borrow()).to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            tail: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[4..16]))),
            buf: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[16..20]))),
        }
    }
}
thread_local!(
    pub static s_0: Value<S> = Rc::new(RefCell::new(S {
        head: Rc::new(RefCell::new(5)),
        tail: Rc::new(RefCell::new(Box::new([0; 3]))),
        buf: Rc::new(RefCell::new(Box::new([0; 4]))),
    }));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((((*(*s_0.with(Value::clone).borrow()).head.borrow()) == 5) as i32) != 0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 3) as i32) != 0) {
        assert!(
            ((((*(*s_0.with(Value::clone).borrow()).tail.borrow())[(*i.borrow()) as usize] == 0)
                as i32)
                != 0)
        );
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
        assert!(
            (((((*(*s_0.with(Value::clone).borrow()).buf.borrow())[(*i.borrow()) as usize] as i32)
                == 0) as i32)
                != 0)
        );
        (*i.borrow_mut()).postfix_inc();
    }
    return 0;
}
