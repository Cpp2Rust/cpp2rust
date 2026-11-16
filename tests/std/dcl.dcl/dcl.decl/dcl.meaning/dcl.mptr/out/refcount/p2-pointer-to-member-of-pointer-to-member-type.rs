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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HoldsSelector {
    #[offset(0)]
    #[byte_size(8)]
    pub selector: MemberPtr<HasTwoInts, i32>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: MemberPtr<HoldsSelector, MemberPtr<HasTwoInts, i32>> =
        member_ptr!(HoldsSelector, selector);
    let holder: Value<HoldsSelector> = Rc::new(RefCell::new(HoldsSelector {
        selector: member_ptr!(HasTwoInts, second),
    }));
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!(
        (((obj.as_pointer())
            .member(((holder.as_pointer()).member(pm).read()))
            .read())
            == 4)
    );
    (holder.as_pointer())
        .member(pm)
        .write(member_ptr!(HasTwoInts, first));
    assert!(
        ({
            (((obj.as_pointer())
                .member(({ (*holder.borrow()).selector }))
                .read())
                == 3)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
