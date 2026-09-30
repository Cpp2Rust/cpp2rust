extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, Default)]
pub struct Holder {
    #[offset(0)]
    pub val: Option<Value<i32>>,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            val: { _a0.with_mut(|__s: &mut Holder| __s.val.take()) },
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for Holder {}
=======
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.val.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: <Option<Value<i32>>>::from_bytes(&buf[0..8]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn read_val_0(h: Ptr<Holder>) -> i32 {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    return (*(*(*h.borrow()).upgrade().deref())
        .val
        .as_ref()
        .unwrap()
        .borrow());
}
pub fn write_val_1(h: Ptr<Holder>, v: i32) {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    (*(*(*h.borrow()).upgrade().deref())
        .val
        .as_ref()
        .unwrap()
        .borrow_mut()) = (*v.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (field_ptr!(h, val) as Ptr<Option<Value<i32>>>).write(Some(Rc::new(RefCell::new(10))).take());
    ({ write_val_1((h.as_pointer()), 42) });
    assert!((({ read_val_0((h.as_pointer()),) }) == 42));
    return 0;
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        (field_ptr!((*self), val) as Ptr<Option<Value<i32>>>)
            .write(_a0.with_mut(|__s: &mut Holder| __s.val.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
