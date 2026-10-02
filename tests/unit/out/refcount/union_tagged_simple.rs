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
#[derive(ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
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
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Event {
    #[offset(0)]
    pub kind: Kind_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub handle: AnyPtr,
    #[offset(16)]
    #[byte_size(8)]
    pub payload: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dummy: Value<i32> = Rc::new(RefCell::new(0));
    let m1: Value<Event> = <Value<Event>>::default();
    (*m1.borrow_mut()).kind = Kind_enum_KIND_DONE;
    (*m1.borrow_mut()).handle = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*m1.borrow_mut()).payload.code().write(42);
    assert!(
        (((({ (*m1.borrow()).kind } as u32) == ((Kind_enum_KIND_DONE as i32) as u32)) as i32) != 0)
    );
    assert!((((((*m1.borrow()).payload.code().read()) == 42) as i32) != 0));
    let m2: Value<Event> = <Value<Event>>::default();
    (*m2.borrow_mut()).kind = Kind_enum_KIND_NONE;
    (*m2.borrow_mut()).handle = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*m2.borrow_mut())
        .payload
        .obj()
        .write(((dummy.as_pointer()) as Ptr<i32>).to_any());
    assert!(
        ((({ ((*m2.borrow()).payload.obj().read()).clone() } == {
            ((dummy.as_pointer()) as Ptr<i32>).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
