extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Vtable {
    pub create: Value<Option<fn(i32) -> AnyPtr>>,
    pub get: Value<Option<fn(AnyPtr) -> i32>>,
    pub destroy: Value<Option<fn(AnyPtr)>>,
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
            create: Rc::new(RefCell::new(None)),
            get: Rc::new(RefCell::new(None)),
            destroy: Rc::new(RefCell::new(None)),
        }
    }
}
impl ByteRepr for Vtable {}
thread_local!(
    pub static storage: Value<i32> = <Value<i32>>::default();
);
pub fn int_create_0(val: i32) -> AnyPtr {
    let val: Value<i32> = Rc::new(RefCell::new(val));
    (*storage.with(Value::clone).borrow_mut()) = (*val.borrow());
    return ((storage.with(Value::clone).as_pointer()) as Ptr<i32>).to_any();
}
pub fn int_get_1(p: AnyPtr) -> i32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).cast::<i32>().expect("ub:wrong type").read());
}
pub fn int_destroy_2(p: AnyPtr) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    (*p.borrow()).cast::<i32>().expect("ub:wrong type").write(0);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vt: Value<Vtable> = Rc::new(RefCell::new(Vtable {
        create: Rc::new(RefCell::new((Some(int_create_0 as _)).clone())),
        get: Rc::new(RefCell::new(Some(int_get_1 as _))),
        destroy: Rc::new(RefCell::new(Some(int_destroy_2 as _))),
    }));
    assert!(!((*(*vt.borrow()).create.borrow()).is_none()));
    assert!(!((*(*vt.borrow()).get.borrow()).is_none()));
    assert!(!((*(*vt.borrow()).destroy.borrow()).is_none()));
    let obj: Value<AnyPtr> = Rc::new(RefCell::new(
        ({
            let _arg0: i32 = 42;
            (*(*vt.borrow()).create.borrow()).unwrap()(_arg0)
        }),
    ));
    assert!(
        (({
            let _arg0: AnyPtr = (*obj.borrow()).clone();
            (*(*vt.borrow()).get.borrow()).unwrap()(_arg0)
        }) == 42)
    );
    ({
        let _arg0: AnyPtr = (*obj.borrow()).clone();
        (*(*vt.borrow()).destroy.borrow()).unwrap()(_arg0)
    });
    assert!(((*storage.with(Value::clone).borrow()) == 0));
    (*(*vt.borrow()).get.borrow_mut()) = None;
    assert!((*(*vt.borrow()).get.borrow()).is_none());
    return 0;
}
