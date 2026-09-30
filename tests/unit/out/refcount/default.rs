extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct Pointers {
    #[offset(0)]
    pub x1: Ptr<i32>,
    #[offset(8)]
    pub x2: Ptr<i32>,
    #[offset(16)]
    pub x3: Value<Box<[Ptr<i32>]>>,
    #[offset(56)]
    pub x4: Value<Box<[Ptr<i32>]>>,
    #[offset(136)]
    pub x5: i32,
}
impl Clone for Pointers {
    fn clone(&self) -> Self {
        Self {
            x1: self.x1.clone(),
            x2: self.x2.clone(),
            x3: Rc::new(RefCell::new((*self.x3.borrow()).clone())),
            x4: Rc::new(RefCell::new((*self.x4.borrow()).clone())),
            x5: self.x5.clone(),
        }
    }
}
impl Default for Pointers {
    fn default() -> Self {
        Pointers {
            x1: Ptr::<i32>::null(),
            x2: Ptr::<i32>::null(),
            x3: Rc::new(RefCell::new(
                (0..5)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
            x4: Rc::new(RefCell::new(
                (0..10)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
            x5: 0_i32,
        }
    }
}
impl ByteRepr for Pointers {
    fn byte_size() -> usize {
        144
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..8]);
        self.x2.to_bytes(&mut buf[8..16]);
        (*self.x3.borrow()).to_bytes(&mut buf[16..56]);
        (*self.x4.borrow()).to_bytes(&mut buf[56..136]);
        self.x5.to_bytes(&mut buf[136..140]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <Ptr<i32>>::from_bytes(&buf[0..8]),
            x2: <Ptr<i32>>::from_bytes(&buf[8..16]),
            x3: Rc::new(RefCell::new(<Box<[Ptr<i32>]>>::from_bytes(&buf[16..56]))),
            x4: Rc::new(RefCell::new(<Box<[Ptr<i32>]>>::from_bytes(&buf[56..136]))),
            x5: <i32>::from_bytes(&buf[136..140]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let default_pointers: Value<Ptr<Pointers>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_usize)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    )));
    (*default_pointers.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
