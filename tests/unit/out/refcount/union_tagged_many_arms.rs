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
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn text(&self) -> Ptr<Ptr<i8>> {
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
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Slot {
    #[offset(0)]
    pub tag: Tag_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub payload: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: Slot = <Slot>::default();
    a.tag = Tag_enum_T_NUM_S;
    a.payload.signed_n().write((-7_i32 as i64));
    assert!(((((a.payload.signed_n().read()) == (-7_i32 as i64)) as i32) != 0));
    let mut b: Slot = <Slot>::default();
    b.tag = Tag_enum_T_NUM_U;
    b.payload.unsigned_n().write(3735928559_u64);
    assert!(((((b.payload.unsigned_n().read()) == 3735928559_u64) as i32) != 0));
    let mut c: Slot = <Slot>::default();
    c.tag = Tag_enum_T_TEXT;
    c.payload
        .text()
        .write(Ptr::<i8>::from_string_literal(b"hello"));
    assert!(
        (((((elem!((c.payload.text().read()), 0).read()) as i32) == ('h' as i32)) as i32) != 0)
    );
    let mut d: Slot = <Slot>::default();
    d.tag = Tag_enum_T_FLOAT;
    d.payload.f().write(1.5E+0);
    assert!(((((d.payload.f().read()) == 1.5E+0) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let mut e: Slot = <Slot>::default();
    e.tag = Tag_enum_T_REF;
    e.payload
        .handle()
        .write(((x.as_pointer()) as Ptr<i32>).to_any());
    assert!(
        ((({ (e.payload.handle().read()) } == { ((x.as_pointer()) as Ptr::<i32>).to_any() })
            as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
