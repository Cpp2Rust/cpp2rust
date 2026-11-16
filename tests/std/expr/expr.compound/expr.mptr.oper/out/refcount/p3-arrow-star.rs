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
    let mut pm: isize = (<HasTwoInts as Record>::OFFSETS.second as isize);
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    let mut p: Ptr<HasTwoInts> = (obj.as_pointer());
    assert!((((p).member::<i32>(pm).read()) == 4));
    (p).member::<i32>(pm).write(5);
    assert!(({ ({ (*obj.borrow()).second } == 5) }));
    assert!(({ ({ (*obj.borrow()).first } == 3) }));
    assert!((((p).member::<i32>(pm)) == ((p).member::<i32>(pm))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
