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
    let source: Value<isize> = Rc::new(RefCell::new(
        (<HasTwoInts as Record>::OFFSETS.second as isize),
    ));
    let destination: Value<isize> = Rc::new(RefCell::new(
        (<HasTwoInts as Record>::OFFSETS.first as isize),
    ));
    match ({ (destination.as_pointer()).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(
                &(source.as_pointer()).to_any(),
                ::std::mem::size_of::<isize>(),
            );
            (__tmp_0).clone()
        }
    };
    assert!(({ ((*destination.borrow()) == (*source.borrow())) }));
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!(
        ({
            (((obj.as_pointer())
                .member::<i32>((*destination.borrow()))
                .read())
                == 4)
        })
    );
    let null_source: Value<isize> = Rc::new(RefCell::new(-1_isize));
    match ({ (destination.as_pointer()).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(
                &(null_source.as_pointer()).to_any(),
                ::std::mem::size_of::<isize>(),
            );
            (__tmp_1).clone()
        }
    };
    assert!(({ ((*destination.borrow()) == -1_isize) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
