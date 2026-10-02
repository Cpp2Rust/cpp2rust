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
pub fn int_create_1(val: i32) -> AnyPtr {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    storage_0.with(|rc| *rc.borrow_mut() = (*val.borrow()));
    return ((storage_0.with(|v| v.as_pointer())) as Ptr<i32>).to_any();
}
pub fn int_get_2(p: AnyPtr) -> i32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).reinterpret_cast::<i32>().read());
}
pub fn int_destroy_3(p: AnyPtr) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    (*p.borrow()).reinterpret_cast::<i32>().write(0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vt: Value<Vtable> = Rc::new(RefCell::new(Vtable {
        create: FnPtr::<fn(i32) -> AnyPtr>::new(int_create_1),
        get: FnPtr::<fn(AnyPtr) -> i32>::new(int_get_2),
        destroy: FnPtr::<fn(AnyPtr)>::new(int_destroy_3),
    }));
    assert!(!(({ (*vt.borrow()).create.clone() }).is_null()));
    assert!(!(({ (*vt.borrow()).get.clone() }).is_null()));
    assert!(!(({ (*vt.borrow()).destroy.clone() }).is_null()));
    let obj: Value<AnyPtr> = Rc::new(RefCell::new(
        ({ { (*vt.borrow()).create.clone() }.call(42) }),
    ));
    assert!((({ { (*vt.borrow()).get.clone() }.call((*obj.borrow()).clone(),) }) == 42));
    ({ { (*vt.borrow()).destroy.clone() }.call((*obj.borrow()).clone()) });
    assert!((storage_0.with(|rc| *rc.borrow()) == 0));
    (*vt.borrow_mut()).get = FnPtr::<fn(AnyPtr) -> i32>::null();
    assert!(({ (*vt.borrow()).get.clone() }).is_null());
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = storage_0.with(|_| ());
}
