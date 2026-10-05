extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Vtable {
    #[offset(0)]
    #[byte_size(8)]
    pub create: FnPtr<fn(i32) -> AnyPtr>,
    #[offset(8)]
    #[byte_size(8)]
    pub get: FnPtr<fn(AnyPtr) -> i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub destroy: FnPtr<fn(AnyPtr)>,
}
thread_local!(
    pub static storage_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn int_create_1(mut val: i32) -> AnyPtr {
    storage_0.with(|rc| *rc.borrow_mut() = val);
    return ((storage_0.with(|v| v.as_pointer())) as Ptr<i32>).to_any();
}
pub fn int_get_2(mut p: AnyPtr) -> i32 {
    return (p.reinterpret_cast::<i32>().read());
}
pub fn int_destroy_3(mut p: AnyPtr) {
    p.reinterpret_cast::<i32>().write(0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut vt: Vtable = Vtable {
        create: FnPtr::<fn(i32) -> AnyPtr>::new(int_create_1),
        get: FnPtr::<fn(AnyPtr) -> i32>::new(int_get_2),
        destroy: FnPtr::<fn(AnyPtr)>::new(int_destroy_3),
    };
    assert!(!((vt.create).is_null()));
    assert!(!((vt.get).is_null()));
    assert!(!((vt.destroy).is_null()));
    let mut obj: AnyPtr = ({ vt.create.call(42) });
    assert!((({ vt.get.call((obj).clone(),) }) == 42));
    ({ vt.destroy.call((obj).clone()) });
    assert!((storage_0.with(|rc| *rc.borrow()) == 0));
    vt.get = FnPtr::<fn(AnyPtr) -> i32>::null();
    assert!((vt.get).is_null());
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = storage_0.with(|_| ());
}
