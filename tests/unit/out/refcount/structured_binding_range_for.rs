extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: Value<i32>,
    pub second: Value<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new((*self.first.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()))),
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
        (*self.first.borrow()).to_bytes(&mut buf[0..4]);
        (*self.second.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            second: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Pair]>> = Rc::new(RefCell::new(Box::new([
        Pair {
            first: Rc::new(RefCell::new(1)),
            second: Rc::new(RefCell::new(2)),
        },
        Pair {
            first: Rc::new(RefCell::new(3)),
            second: Rc::new(RefCell::new(4)),
        },
        Pair {
            first: Rc::new(RefCell::new(5)),
            second: Rc::new(RefCell::new(6)),
        },
    ])));
    'loop_: for mut __decomp_0 in arr.as_pointer() as Ptr<Pair> {
        (*(*__decomp_0.upgrade().deref()).first.borrow_mut()) +=
            (*(*__decomp_0.upgrade().deref()).second.borrow());
    }
    assert!(((*(*arr.borrow())[(0) as usize].first.borrow()) == 3));
    assert!(((*(*arr.borrow())[(2) as usize].first.borrow()) == 11));
    'loop_: for mut __decomp_1 in arr.as_pointer() as Ptr<Pair> {
        let __decomp_1: Value<Pair> = Rc::new(RefCell::new(__decomp_1.upgrade().deref().clone()));
        (*(*__decomp_1.borrow()).first.borrow_mut()) = 0;
        (*(*__decomp_1.borrow()).second.borrow_mut()) = 0;
    }
    assert!(((*(*arr.borrow())[(1) as usize].first.borrow()) == 7));
    assert!(((*(*arr.borrow())[(1) as usize].second.borrow()) == 4));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut __decomp_2 in arr.as_pointer() as Ptr<Pair> {
        (*sum.borrow_mut()) += ((*(*__decomp_2.upgrade().deref()).first.borrow())
            * (*(*__decomp_2.upgrade().deref()).second.borrow()));
    }
    assert!(((*sum.borrow()) == (((3 * 2) + (7 * 4)) + (11 * 6))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
