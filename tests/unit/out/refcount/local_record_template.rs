extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(t: Local) -> i32 {
    let t: Value<Local> = Rc::new(RefCell::new(t));
    return (*(*t.borrow()).x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let l: Value<Local> = Rc::new(RefCell::new(Local {
        x: Rc::new(RefCell::new(7)),
    }));
    assert!((({ get_0((*l.borrow()).clone(),) }) == 7));
    return 0;
}
#[derive(Default)]
pub struct Local {
    pub x: Value<i32>,
}
impl Clone for Local {
    fn clone(&self) -> Self {
        let __this: Value<Local> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Local> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Local {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
