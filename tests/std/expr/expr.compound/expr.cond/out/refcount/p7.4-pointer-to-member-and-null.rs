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
pub fn second_or_null_0(mut choose_second: bool) -> isize {
    return if choose_second {
        (offset_of_field!(HasTwoInts, second) as isize)
    } else {
        -1_isize
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!((({ second_or_null_0(false,) }) == -1_isize));
    assert!((({ second_or_null_0(true,) }) == (offset_of_field!(HasTwoInts, second) as isize)));
    assert!(
        (((obj.as_pointer())
            .member::<i32>(({ second_or_null_0(true,) }))
            .read())
            == 4)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
