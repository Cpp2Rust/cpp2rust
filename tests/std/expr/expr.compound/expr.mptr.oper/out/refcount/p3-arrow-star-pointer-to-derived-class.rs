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
pub struct Precedes {
    #[offset(0)]
    pub preceding: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DeclaresValue {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct InheritsValue {
    #[offset(8)]
    pub own: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: MemberPtr<DeclaresValue, i32> = member_ptr!(DeclaresValue, value);
    let obj: Value<InheritsValue> = Rc::new(RefCell::new(<InheritsValue>::default()));
    ((*obj.borrow_mut()) as Precedes).preceding = 1;
    ((*obj.borrow_mut()) as DeclaresValue).value = 2;
    (*obj.borrow_mut()).own = 3;
    let mut p: Ptr<InheritsValue> = (obj.as_pointer());
    assert!((((p).member(pm).read()) == 2));
    (p).member(pm).write(4);
    assert!(({ ({ ((*obj.borrow()) as DeclaresValue).value } == 4) }));
    assert!(({ ({ ((*obj.borrow()) as Precedes).preceding } == 1) }));
    assert!(({ ({ (*obj.borrow()).own } == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
