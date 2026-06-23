extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct NonCopy {
    pub data: Value<Vec<i32>>,
    pub tag: Value<i32>,
}
impl Clone for NonCopy {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
        };
        this
    }
}
impl ByteRepr for NonCopy {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..24]);
        (*self.tag.borrow()).to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[0..24]))),
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[24..28]))),
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
    (*(*arr.borrow())[(0) as usize].tag.borrow_mut()) = 7;
    (*(*arr.borrow())[(1) as usize].data.borrow_mut()).push(42);
    assert!(((*(*arr.borrow())[(0) as usize].tag.borrow()) == 7));
    assert!(((*(*arr.borrow())[(1) as usize].data.borrow()).len() == 1_usize));
    assert!(
        ((((*arr.borrow())[(1) as usize].data.as_pointer() as Ptr<i32>)
            .offset(0_usize as isize)
            .read())
            == 42)
    );
    assert!(((*(*arr.borrow())[(2) as usize].tag.borrow()) == 0));
    assert!(((*(*arr.borrow())[(2) as usize].data.borrow()).len() == 0_usize));
    return 0;
}
