extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub v: Value<Vec<i32>>,
    #[offset(24)]
    pub a: i32,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: { Rc::new(RefCell::new((*self.v.borrow()).clone())) },
            a: { self.a },
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for S {}
=======
impl ByteRepr for S {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..24]);
        self.a.to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <Value<Vec<i32>>>::from_bytes(&buf[0..24]),
            a: <i32>::from_bytes(&buf[24..28]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*(*s.borrow()).v.borrow_mut()).push(1);
    'loop_: for mut e in { (*s.borrow()).v.as_pointer() } as Ptr<i32> {
        let e: Value<i32> = Rc::new(RefCell::new(e.read()));
        (*s.borrow_mut()).a.postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
