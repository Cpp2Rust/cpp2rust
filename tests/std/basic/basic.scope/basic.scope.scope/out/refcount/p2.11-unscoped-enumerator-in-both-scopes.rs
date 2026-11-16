extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Direction = u32;
pub const Direction_north: Direction = 4;
pub const Direction_south: Direction = 9;
pub type HasUnscopedEnum_Size = u32;
pub const HasUnscopedEnum_Size_small: HasUnscopedEnum_Size = 3;
pub const HasUnscopedEnum_Size_large: HasUnscopedEnum_Size = 8;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasUnscopedEnum {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((Direction_north as i32) == 4));
    assert!(((Direction_north as i32) == 4));
    assert!(((Direction_south as i32) == 9));
    assert!(((Direction_south as i32) == 9));
    assert!(((HasUnscopedEnum_Size_small as i32) == 3));
    assert!(((HasUnscopedEnum_Size_small as i32) == 3));
    assert!(((HasUnscopedEnum_Size_large as i32) == 8));
    assert!(((HasUnscopedEnum_Size_large as i32) == 8));
    let object: Value<HasUnscopedEnum> = Rc::new(RefCell::new(<HasUnscopedEnum>::default()));
    assert!((({ HasUnscopedEnumImpl::scale(&object.as_pointer(), 2,) }) == 16));
    assert!(((HasUnscopedEnum_Size_small as i32) == 3));
    return 0;
}
pub trait HasUnscopedEnumImpl {
    fn scale(&self, count: i32) -> i32;
}
impl HasUnscopedEnumImpl for Ptr<HasUnscopedEnum> {
    fn scale(&self, mut count: i32) -> i32 {
        return (count * (HasUnscopedEnum_Size_large as i32));
    }
}
pub fn __cpp2rust_init_globals() {}
