extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(5)));
    let out: Value<i32> = Rc::new(RefCell::new(((*x.borrow()).read())));
    (*x.borrow()).delete();
    assert!(((*out.borrow()) == 5));
    let y: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(Default::default())));
    (*y.borrow()).write(9);
    assert!((((*y.borrow()).read()) == 9));
    (*y.borrow()).delete();
    let p: Value<Ptr<Pair>> = Rc::new(RefCell::new(Ptr::alloc(Pair {
        x: Rc::new(RefCell::new(0_i32)),
        y: Rc::new(RefCell::new(0_i32)),
    })));
    (*(*(*p.borrow()).upgrade().deref()).x.borrow_mut()) = 1;
    (*(*(*p.borrow()).upgrade().deref()).y.borrow_mut()) = 2;
    assert!(
        ({
            let _lhs = (*(*(*p.borrow()).upgrade().deref()).x.borrow());
            _lhs + (*(*(*p.borrow()).upgrade().deref()).y.borrow())
        } == 3)
    );
    (*p.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
