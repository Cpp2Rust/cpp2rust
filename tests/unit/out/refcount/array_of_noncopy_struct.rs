extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct NonCopy {
    pub data: Vec<i32>,
    pub tag: i32,
}
impl Clone for NonCopy {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: (self.data).clone(),
            tag: self.tag,
        };
        this
    }
}
impl ByteRepr for NonCopy {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..24]);
        self.tag.to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <Vec<i32>>::from_bytes(&buf[0..24]),
            tag: <i32>::from_bytes(&buf[24..28]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[NonCopy]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| <NonCopy>::default())
            .collect::<Box<[NonCopy]>>(),
    ));
    (*arr.borrow_mut())[(0) as usize].tag = 7;
    (*arr.borrow_mut())[(1) as usize].data.push(42);
    assert!(((*arr.borrow())[(0) as usize].tag == 7));
    assert!(((*arr.borrow())[(1) as usize].data.len() == 1_usize));
    assert!(
        ((((arr.as_pointer() as Ptr<NonCopy>).offset(1).field_ptr(
            0,
            |__v: &NonCopy| &__v.data[..],
            |__v: &mut NonCopy| &mut __v.data[..]
        ) as Ptr<i32>)
            .offset(0_usize)
            .read())
            == 42)
    );
    assert!(((*arr.borrow())[(2) as usize].tag == 0));
    assert!(((*arr.borrow())[(2) as usize].data.len() == 0_usize));
    return 0;
}
