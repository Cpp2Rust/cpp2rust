extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Wrapper_Probe_ {
    pub base_: Value<Probe>,
    pub tag: Value<i32>,
}
impl Clone for Wrapper_Probe_ {
    fn clone(&self) -> Self {
        let __this: Value<Wrapper_Probe_> = Rc::new(RefCell::new(Self {
            base_: Rc::new(RefCell::new((*self.base_.borrow()).clone())),
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
        }));
        let this: Ptr<Wrapper_Probe_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Wrapper_Probe_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_.borrow()).to_bytes(&mut buf[0..1]);
        (*self.tag.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_: Rc::new(RefCell::new(<Probe>::from_bytes(&buf[0..1]))),
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Wrapper_Probe_> = Rc::new(RefCell::new(Wrapper_Probe_ {
        base_: Rc::new(RefCell::new(Probe {})),
        tag: Rc::new(RefCell::new(<i32>::default())),
    }));
    (*(*a.borrow()).tag.borrow_mut()) = 3;
    let b: Value<Wrapper_Probe_> = Rc::new(RefCell::new((*a.borrow()).clone()));
    assert!(((*(*b.borrow()).tag.borrow()) == 3));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct Probe;
