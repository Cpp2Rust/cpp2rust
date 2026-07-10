extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Tag_enum {
    #[default]
    T_NUM_S = 0,
    T_NUM_U = 1,
    T_TEXT = 2,
    T_FLOAT = 3,
    T_REF = 4,
}
impl From<i32> for Tag_enum {
    fn from(n: i32) -> Tag_enum {
        match n {
            0 => Tag_enum::T_NUM_S,
            1 => Tag_enum::T_NUM_U,
            2 => Tag_enum::T_TEXT,
            3 => Tag_enum::T_FLOAT,
            4 => Tag_enum::T_REF,
            _ => panic!("invalid Tag_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Tag_enum);
impl ByteRepr for Tag_enum {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <Tag_enum>::from(i32::from_bytes(buf))
    }
}
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
    (*(*a.borrow()).tag.borrow_mut()) = Tag_enum::T_NUM_S;
    (*(*a.borrow()).payload.borrow_mut())
        .signed_n()
        .write((-7_i32 as i64));
    assert!(
        (((((*(*a.borrow()).payload.borrow()).signed_n().read()) == (-7_i32 as i64)) as i32) != 0)
    );
    let b: Value<Slot> = <Value<Slot>>::default();
    (*(*b.borrow()).tag.borrow_mut()) = Tag_enum::T_NUM_U;
    (*(*b.borrow()).payload.borrow_mut())
        .unsigned_n()
        .write(3735928559_u64);
    assert!(
        (((((*(*b.borrow()).payload.borrow()).unsigned_n().read()) == 3735928559_u64) as i32) != 0)
    );
    let c: Value<Slot> = <Value<Slot>>::default();
    (*(*c.borrow()).tag.borrow_mut()) = Tag_enum::T_TEXT;
    (*(*c.borrow()).payload.borrow_mut())
        .text()
        .write((Ptr::from_string_literal(b"hello")).clone());
    assert!(
        (((((((*(*c.borrow()).payload.borrow()).text().read())
            .offset((0) as isize)
            .read()) as i32)
            == ('h' as i32)) as i32)
            != 0)
    );
    let d: Value<Slot> = <Value<Slot>>::default();
    (*(*d.borrow()).tag.borrow_mut()) = Tag_enum::T_FLOAT;
    (*(*d.borrow()).payload.borrow_mut()).f().write(1.5E+0);
    assert!((((((*(*d.borrow()).payload.borrow()).f().read()) == 1.5E+0) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let e: Value<Slot> = <Value<Slot>>::default();
    (*(*e.borrow()).tag.borrow_mut()) = Tag_enum::T_REF;
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
