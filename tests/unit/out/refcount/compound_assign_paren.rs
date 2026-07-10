extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Item {
    pub flags: Value<u8>,
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let mut this = Self {
            flags: Rc::new(RefCell::new((*self.flags.borrow()))),
        };
        this
    }
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.flags.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            flags: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let item: Value<Item> = Rc::new(RefCell::new(Item {
        flags: Rc::new(RefCell::new(0_u8)),
    }));
    let ptr: Value<Ptr<Item>> = Rc::new(RefCell::new((item.as_pointer())));
    ({
        let rhs_0 =
            (((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32) | (1 << 0)) as u8;
        (*(*(*ptr.borrow()).upgrade().deref()).flags.borrow_mut()) = rhs_0
    });
    ({
        let rhs_0 =
            (((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32) | (1 << 1)) as u8;
        (*(*(*ptr.borrow()).upgrade().deref()).flags.borrow_mut()) = rhs_0
    });
    assert!((((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32) == 3));
    ({
        let rhs_0 = (((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32)
            & ((!(1 << 0) as u8) as i32)) as u8;
        (*(*(*ptr.borrow()).upgrade().deref()).flags.borrow_mut()) = rhs_0
    });
    assert!((((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32) == 2));
    let bits: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    ({
        let rhs_0 = (((*bits.borrow())[((5) / 8) as usize] as i32)
            | (((1 << ((5) & 7)) as u8) as i32)) as u8;
        (*bits.borrow_mut())[((5) / 8) as usize] = rhs_0
    });
    ({
        let rhs_0 = (((*bits.borrow())[((13) / 8) as usize] as i32)
            | (((1 << ((13) & 7)) as u8) as i32)) as u8;
        (*bits.borrow_mut())[((13) / 8) as usize] = rhs_0
    });
    assert!((((*bits.borrow())[(0) as usize] as i32) == 32));
    assert!((((*bits.borrow())[(1) as usize] as i32) == 32));
    assert!((((*bits.borrow())[(2) as usize] as i32) == 0));
    if (((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32) != 0) {
        ({
            let rhs_0 = (((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32)
                & ((!(1 << 1) as u8) as i32)) as u8;
            (*(*(*ptr.borrow()).upgrade().deref()).flags.borrow_mut()) = rhs_0
        });
    }
    assert!((((*(*(*ptr.borrow()).upgrade().deref()).flags.borrow()) as i32) == 0));
    return 0;
}
