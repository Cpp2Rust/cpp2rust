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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct HoldsSelector {
    #[offset(0)]
    #[byte_size(8)]
    pub selector: isize,
}
impl Default for HoldsSelector {
    fn default() -> Self {
        { HoldsSelector { selector: -1_isize } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: isize = (<HoldsSelector as Record>::OFFSETS.selector as isize);
    let holder: Value<HoldsSelector> = Rc::new(RefCell::new(HoldsSelector {
        selector: (<HasTwoInts as Record>::OFFSETS.second as isize),
    }));
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(HasTwoInts {
        first: 3,
        second: 4,
    }));
    assert!(
        (((obj.as_pointer())
            .member::<i32>(((holder.as_pointer()).member::<isize>(pm).read()))
            .read())
            == 4)
    );
    (holder.as_pointer())
        .member::<isize>(pm)
        .write((<HasTwoInts as Record>::OFFSETS.first as isize));
    assert!(
        ({
            (((obj.as_pointer())
                .member::<i32>(({ (*holder.borrow()).selector }))
                .read())
                == 3)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
