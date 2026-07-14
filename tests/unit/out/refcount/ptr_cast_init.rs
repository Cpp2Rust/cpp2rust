extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct header {
    pub tag: Value<i32>,
    pub size: Value<i32>,
}
impl Clone for header {
    fn clone(&self) -> Self {
        Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()).clone())),
            size: Rc::new(RefCell::new((*self.size.borrow()).clone())),
        }
    }
}
impl ByteRepr for header {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.size.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct view {
    pub tag: Value<i32>,
}
impl Clone for view {
    fn clone(&self) -> Self {
        Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()).clone())),
        }
    }
}
impl ByteRepr for view {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let text: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
    let cp: Value<Ptr<u8>> = Rc::new(RefCell::new((text.as_pointer() as Ptr<u8>)));
    let u: Value<Ptr<u8>> = Rc::new(RefCell::new((*cp.borrow()).reinterpret_cast::<u8>()));
    assert!(((((((*u.borrow()).offset((0) as isize).read()) as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((((*u.borrow()).offset((1) as isize).read()) as i32) == ('i' as i32)) as i32) != 0));
    let h: Value<header> = Rc::new(RefCell::new(header {
        tag: Rc::new(RefCell::new(7)),
        size: Rc::new(RefCell::new(32)),
    }));
    let hp: Value<Ptr<header>> = Rc::new(RefCell::new((h.as_pointer())));
    let v: Value<Ptr<view>> = Rc::new(RefCell::new((*hp.borrow()).reinterpret_cast::<view>()));
    assert!(((((*(*(*v.borrow()).upgrade().deref()).tag.borrow()) == 7) as i32) != 0));
    let data: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
    let vp: Value<AnyPtr> = Rc::new(RefCell::new(
        ((data.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
    ));
    let n: Value<i32> = Rc::new(RefCell::new(2));
    let sel: Value<Ptr<u8>> = Rc::new(RefCell::new(
        if ((((*n.borrow()) < 100) as i32) != 0) {
            (*vp.borrow()).clone()
        } else {
            (AnyPtr::default())
        }
        .reinterpret_cast::<u8>(),
    ));
    assert!((((!((*sel.borrow()).is_null())) as i32) != 0));
    assert!(
        ((((((*sel.borrow()).offset((0) as isize).read()) as i32) == ('h' as i32)) as i32) != 0)
    );
    (*n.borrow_mut()) = 200;
    (*sel.borrow_mut()) = if ((((*n.borrow()) < 100) as i32) != 0) {
        (*vp.borrow()).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<u8>();
    assert!(((((*sel.borrow()).is_null()) as i32) != 0));
    return 0;
}
