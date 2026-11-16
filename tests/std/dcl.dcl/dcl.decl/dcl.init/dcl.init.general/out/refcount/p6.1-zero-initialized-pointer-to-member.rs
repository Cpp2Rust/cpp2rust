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
thread_local!(
    pub static static_pm_0: Value<MemberPtr<HasTwoInts, i32>> =
        <Value<MemberPtr<HasTwoInts, i32>>>::default();
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((static_pm_0.with(|rc| *rc.borrow())).is_null());
    assert!((static_pm_0.with(|rc| *rc.borrow()) != member_ptr!(HasTwoInts, first)));
    let mut value_initialized: MemberPtr<HasTwoInts, i32> = <MemberPtr<HasTwoInts, i32>>::default();
    assert!((value_initialized).is_null());
    assert!((value_initialized != member_ptr!(HasTwoInts, first)));
    let mut holder: HoldsSelector = <HoldsSelector>::default();
    assert!((holder.selector).is_null());
    assert!((holder.selector != member_ptr!(HasTwoInts, first)));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = static_pm_0.with(|_| ());
}
