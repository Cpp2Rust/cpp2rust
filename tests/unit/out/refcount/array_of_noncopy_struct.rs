extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct NonCopy {
    #[offset(0)]
    pub data: Value<Vec<i32>>,
    #[offset(24)]
    pub tag: i32,
}
impl Clone for NonCopy {
    fn clone(&self) -> Self {
        let __this: Value<NonCopy> = Rc::new(RefCell::new(Self {
            data: { Rc::new(RefCell::new((*self.data.borrow()).clone())) },
            tag: { self.tag },
        }));
        let this: Ptr<NonCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for NonCopy {
    fn default() -> Self {
        NonCopy {
            data: Rc::new(RefCell::new(Default::default())),
            tag: 0,
        }
    }
}
<<<<<<< HEAD
impl ByteRepr for NonCopy {}
=======
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
            data: <Value<Vec<i32>>>::from_bytes(&buf[0..24]),
            tag: <i32>::from_bytes(&buf[24..28]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[NonCopy]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| <NonCopy>::default())
            .collect::<Box<[NonCopy]>>(),
    ));
    (*arr.borrow_mut())[(0) as usize].tag = 7;
    (*(*arr.borrow())[(1) as usize].data.borrow_mut()).push(42);
    assert!(({ (*arr.borrow())[(0) as usize].tag } == 7));
    assert!(((*(*arr.borrow())[(1) as usize].data.borrow()).len() == 1_usize));
    assert!(({ (*(*arr.borrow())[(1) as usize].data.borrow())[(0_usize) as usize] } == 42));
    assert!(({ (*arr.borrow())[(2) as usize].tag } == 0));
    assert!(((*(*arr.borrow())[(2) as usize].data.borrow()).len() == 0_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
