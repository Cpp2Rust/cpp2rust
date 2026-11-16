extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DeclaresValue {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct InheritsValue {
    #[offset(4)]
    pub own: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: MemberPtr<DeclaresValue, i32> = member_ptr!(DeclaresValue, value);
    let base: Value<DeclaresValue> = Rc::new(RefCell::new(DeclaresValue { value: 3 }));
    assert!((((base.as_pointer()).member(pm).read()) == 3));
    let derived: Value<InheritsValue> = Rc::new(RefCell::new(InheritsValue {
        own: DeclaresValue { value: 4 },
    }));
    assert!((((derived.as_pointer()).member(pm).read()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
