extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Tag_enum = u32;
pub const Tag_enum_T_NUM_S: Tag_enum = 0;
pub const Tag_enum_T_NUM_U: Tag_enum = 1;
pub const Tag_enum_T_TEXT: Tag_enum = 2;
pub const Tag_enum_T_FLOAT: Tag_enum = 3;
pub const Tag_enum_T_REF: Tag_enum = 4;
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn text(&self) -> Ptr<Ptr<u8>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn handle(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn signed_n(&self) -> Ptr<i64> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn unsigned_n(&self) -> Ptr<u64> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn f(&self) -> Ptr<f64> {
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
pub struct Slot {
    pub tag: Value<Tag_enum>,
    pub payload: Value<anon_0>,
}
impl Clone for Slot {
    fn clone(&self) -> Self {
        Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()).clone())),
            payload: Rc::new(RefCell::new((*self.payload.borrow()).clone())),
        }
    }
}
impl ByteRepr for Slot {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.payload.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<Tag_enum>::from_bytes(&buf[0..4]))),
            payload: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Slot> = <Value<Slot>>::default();
    (*(*a.borrow()).tag.borrow_mut()) = Tag_enum_T_NUM_S;
    (*(*a.borrow()).payload.borrow_mut())
        .signed_n()
        .write((-7_i32 as i64));
    assert!(
        (((((*(*a.borrow()).payload.borrow()).signed_n().read()) == (-7_i32 as i64)) as i32) != 0)
    );
    let b: Value<Slot> = <Value<Slot>>::default();
    (*(*b.borrow()).tag.borrow_mut()) = Tag_enum_T_NUM_U;
    (*(*b.borrow()).payload.borrow_mut())
        .unsigned_n()
        .write(3735928559_u64);
    assert!(
        (((((*(*b.borrow()).payload.borrow()).unsigned_n().read()) == 3735928559_u64) as i32) != 0)
    );
    let c: Value<Slot> = <Value<Slot>>::default();
    (*(*c.borrow()).tag.borrow_mut()) = Tag_enum_T_TEXT;
    (*(*c.borrow()).payload.borrow_mut())
        .text()
        .write(Ptr::from_string_literal(b"hello"));
    assert!(
        (((((((*(*c.borrow()).payload.borrow()).text().read())
            .offset((0) as isize)
            .read()) as i32)
            == ('h' as i32)) as i32)
            != 0)
    );
    let d: Value<Slot> = <Value<Slot>>::default();
    (*(*d.borrow()).tag.borrow_mut()) = Tag_enum_T_FLOAT;
    (*(*d.borrow()).payload.borrow_mut()).f().write(1.5E+0);
    assert!((((((*(*d.borrow()).payload.borrow()).f().read()) == 1.5E+0) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let e: Value<Slot> = <Value<Slot>>::default();
    (*(*e.borrow()).tag.borrow_mut()) = Tag_enum_T_REF;
    (*(*e.borrow()).payload.borrow_mut())
        .handle()
        .write(((x.as_pointer()) as Ptr<i32>).to_any());
    assert!(
        ((({
            let _lhs = ((*(*e.borrow()).payload.borrow()).handle().read()).clone();
            _lhs == ((x.as_pointer()) as Ptr<i32>).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
