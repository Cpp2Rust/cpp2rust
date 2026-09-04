extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Kind_enum = u32;
pub const Kind_enum_KIND_NONE: Kind_enum = 0;
pub const Kind_enum_KIND_DONE: Kind_enum = 1;
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn obj(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn code(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[derive(Default)]
pub struct Event {
    pub kind: Value<Kind_enum>,
    pub handle: Value<AnyPtr>,
    pub payload: Value<anon_0>,
}
impl Clone for Event {
    fn clone(&self) -> Self {
        Self {
            kind: Rc::new(RefCell::new((*self.kind.borrow()).clone())),
            handle: Rc::new(RefCell::new((*self.handle.borrow()).clone())),
            payload: Rc::new(RefCell::new((*self.payload.borrow()).clone())),
        }
    }
}
impl ByteRepr for Event {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.kind.borrow()).to_bytes(&mut buf[0..4]);
        (*self.handle.borrow()).to_bytes(&mut buf[8..16]);
        (*self.payload.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: Rc::new(RefCell::new(<Kind_enum>::from_bytes(&buf[0..4]))),
            handle: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[8..16]))),
            payload: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[16..24]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dummy: Value<i32> = Rc::new(RefCell::new(0));
    let m1: Value<Event> = <Value<Event>>::default();
    (*(*m1.borrow()).kind.borrow_mut()) = Kind_enum_KIND_DONE;
    (*(*m1.borrow()).handle.borrow_mut()) = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*(*m1.borrow()).payload.borrow_mut()).code().write(42);
    assert!(
        (((((*(*m1.borrow()).kind.borrow()) as u32) == ((Kind_enum_KIND_DONE as i32) as u32))
            as i32)
            != 0)
    );
    assert!((((((*(*m1.borrow()).payload.borrow()).code().read()) == 42) as i32) != 0));
    let m2: Value<Event> = <Value<Event>>::default();
    (*(*m2.borrow()).kind.borrow_mut()) = Kind_enum_KIND_NONE;
    (*(*m2.borrow()).handle.borrow_mut()) = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*(*m2.borrow()).payload.borrow_mut())
        .obj()
        .write(((dummy.as_pointer()) as Ptr<i32>).to_any());
    assert!(
        ((({
            let _lhs = ((*(*m2.borrow()).payload.borrow()).obj().read()).clone();
            _lhs == ((dummy.as_pointer()) as Ptr<i32>).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
