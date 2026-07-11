extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Vtable {
    pub create: Value<FnPtr<fn(i32) -> AnyPtr>>,
    pub get: Value<FnPtr<fn(AnyPtr) -> i32>>,
    pub destroy: Value<FnPtr<fn(AnyPtr)>>,
}
impl Clone for Vtable {
    fn clone(&self) -> Self {
        let mut this = Self {
            create: Rc::new(RefCell::new((*self.create.borrow()).clone())),
            get: Rc::new(RefCell::new((*self.get.borrow()).clone())),
            destroy: Rc::new(RefCell::new((*self.destroy.borrow()).clone())),
        };
        this
    }
}
impl Default for Vtable {
    fn default() -> Self {
        Vtable {
            create: Rc::new(RefCell::new(FnPtr::null())),
            get: Rc::new(RefCell::new(FnPtr::null())),
            destroy: Rc::new(RefCell::new(FnPtr::null())),
        }
    }
}
impl ByteRepr for Vtable {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.create.borrow()).to_bytes(&mut buf[0..8]);
        (*self.get.borrow()).to_bytes(&mut buf[8..16]);
        (*self.destroy.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            create: Rc::new(RefCell::new(<FnPtr<fn(i32) -> AnyPtr>>::from_bytes(
                &buf[0..8],
            ))),
            get: Rc::new(RefCell::new(<FnPtr<fn(AnyPtr) -> i32>>::from_bytes(
                &buf[8..16],
            ))),
            destroy: Rc::new(RefCell::new(<FnPtr<fn(AnyPtr)>>::from_bytes(&buf[16..24]))),
        }
    }
}
thread_local!(
    pub static storage_0: Value<i32> = <Value<i32>>::default();
);
pub fn int_create_1(val: i32) -> AnyPtr {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    (*storage_0.with(Value::clone).borrow_mut()) = (*val.borrow());
    return ((storage_0.with(Value::clone).as_pointer()) as Ptr<i32>).to_any();
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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vt: Value<Vtable> = Rc::new(RefCell::new(Vtable {
        create: Rc::new(RefCell::new(FnPtr::<fn(i32) -> AnyPtr>::new(int_create_1))),
        get: Rc::new(RefCell::new(FnPtr::<fn(AnyPtr) -> i32>::new(int_get_2))),
        destroy: Rc::new(RefCell::new(FnPtr::<fn(AnyPtr)>::new(int_destroy_3))),
    }));
    assert!(!((*(*vt.borrow()).create.borrow()).is_null()));
    assert!(!((*(*vt.borrow()).get.borrow()).is_null()));
    assert!(!((*(*vt.borrow()).destroy.borrow()).is_null()));
    let obj: Value<AnyPtr> = Rc::new(RefCell::new(({ (*(*(*vt.borrow()).create.borrow()))(42) })));
    assert!((({ (*(*(*vt.borrow()).get.borrow()))((*obj.borrow()).clone(),) }) == 42));
    ({ (*(*(*vt.borrow()).destroy.borrow()))((*obj.borrow()).clone()) });
    assert!(((*storage_0.with(Value::clone).borrow()) == 0));
    (*(*vt.borrow()).get.borrow_mut()) = FnPtr::null();
    assert!((*(*vt.borrow()).get.borrow()).is_null());
    return 0;
}
