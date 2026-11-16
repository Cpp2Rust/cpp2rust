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
pub struct HasTwoInts {
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
    let mut pm: MemberPtr<HasTwoInts, i32> = member_ptr!(HasTwoInts, first);
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!((((obj.as_pointer()).member(pm).read()) == 3));
    (obj.as_pointer()).member(pm).write(5);
    assert!(({ ({ (*obj.borrow()).first } == 5) }));
    assert!(({ ({ (*obj.borrow()).second } == 4) }));
    pm = member_ptr!(HasTwoInts, second);
    assert!((((obj.as_pointer()).member(pm).read()) == 4));
    (obj.as_pointer()).member(pm).write(6);
    assert!(({ ({ (*obj.borrow()).first } == 5) }));
    assert!(({ ({ (*obj.borrow()).second } == 6) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
