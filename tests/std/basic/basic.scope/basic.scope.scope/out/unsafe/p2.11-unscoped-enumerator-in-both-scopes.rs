extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Direction = u32;
pub const Direction_north: Direction = 4;
pub const Direction_south: Direction = 9;
pub type HasUnscopedEnum_Size = u32;
pub const HasUnscopedEnum_Size_small: HasUnscopedEnum_Size = 3;
pub const HasUnscopedEnum_Size_large: HasUnscopedEnum_Size = 8;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasUnscopedEnum {}
impl HasUnscopedEnum {
    pub unsafe fn scale(&self, mut count: i32) -> i32 {
        return ((count) * (HasUnscopedEnum_Size_large as i32));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((Direction_north as i32) == (4)));
    assert!(((Direction_north as i32) == (4)));
    assert!(((Direction_south as i32) == (9)));
    assert!(((Direction_south as i32) == (9)));
    assert!(((HasUnscopedEnum_Size_small as i32) == (3)));
    assert!(((HasUnscopedEnum_Size_small as i32) == (3)));
    assert!(((HasUnscopedEnum_Size_large as i32) == (8)));
    assert!(((HasUnscopedEnum_Size_large as i32) == (8)));
    let mut object: HasUnscopedEnum = <HasUnscopedEnum>::default();
    assert!(((unsafe { HasUnscopedEnum::scale(&object, 2,) }) == (16)));
    assert!(((HasUnscopedEnum_Size_small as i32) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
