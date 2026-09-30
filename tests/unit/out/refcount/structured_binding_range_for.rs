extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Pair]>> = Rc::new(RefCell::new(Box::new([
        Pair {
            first: 1,
            second: 2,
        },
        Pair {
            first: 3,
            second: 4,
        },
        Pair {
            first: 5,
            second: 6,
        },
    ])));
    'loop_: for mut __decomp_0 in arr.as_pointer() as Ptr<Pair> {
        {
            let _ptr = field!(__decomp_0, first);
            _ptr.write(_ptr.read() + __decomp_0.with(|__s| __s.second))
        };
    }
    assert!(({ (*arr.borrow())[(0) as usize].first } == 3));
    assert!(({ (*arr.borrow())[(2) as usize].first } == 11));
    'loop_: for mut __decomp_1 in arr.as_pointer() as Ptr<Pair> {
        let __decomp_1: Value<Pair> = Rc::new(RefCell::new(__decomp_1.upgrade().deref().clone()));
        (*__decomp_1.borrow_mut()).first = 0;
        (*__decomp_1.borrow_mut()).second = 0;
    }
    assert!(({ (*arr.borrow())[(1) as usize].first } == 7));
    assert!(({ (*arr.borrow())[(1) as usize].second } == 4));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut __decomp_2 in arr.as_pointer() as Ptr<Pair> {
        (*sum.borrow_mut()) +=
            (__decomp_2.with(|__s| __s.first) * __decomp_2.with(|__s| __s.second));
    }
    assert!(((*sum.borrow()) == (((3 * 2) + (7 * 4)) + (11 * 6))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
