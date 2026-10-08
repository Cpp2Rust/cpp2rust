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
    pub fn text(this: Ptr<Self>) -> Ptr<Ptr<i8>> {
        this.reinterpret_cast()
    }
    pub fn handle(this: Ptr<Self>) -> Ptr<AnyPtr> {
        this.reinterpret_cast()
    }
    pub fn signed_n(this: Ptr<Self>) -> Ptr<i64> {
        this.reinterpret_cast()
    }
    pub fn unsigned_n(this: Ptr<Self>) -> Ptr<u64> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
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
    let a: Value<Slot> = <Value<Slot>>::default();
    (*a.borrow_mut()).tag = Tag_enum_T_NUM_S;
    anon_0::signed_n(field_ptr!(a.as_pointer(), payload)).write((-7_i32 as i64));
    assert!(
        ((((anon_0::signed_n(field_ptr!(a.as_pointer(), payload)).read()) == (-7_i32 as i64))
            as i32)
            != 0)
    );
    let b: Value<Slot> = <Value<Slot>>::default();
    (*b.borrow_mut()).tag = Tag_enum_T_NUM_U;
    anon_0::unsigned_n(field_ptr!(b.as_pointer(), payload)).write(3735928559_u64);
    assert!(
        ((((anon_0::unsigned_n(field_ptr!(b.as_pointer(), payload)).read()) == 3735928559_u64)
            as i32)
            != 0)
    );
    let c: Value<Slot> = <Value<Slot>>::default();
    (*c.borrow_mut()).tag = Tag_enum_T_TEXT;
    anon_0::text(field_ptr!(c.as_pointer(), payload))
        .write(Ptr::<i8>::from_string_literal(b"hello"));
    assert!(
        (((((elem!(
            (anon_0::text(field_ptr!(c.as_pointer(), payload)).read()),
            0
        )
        .read()) as i32)
            == ('h' as i32)) as i32)
            != 0)
    );
    let d: Value<Slot> = <Value<Slot>>::default();
    (*d.borrow_mut()).tag = Tag_enum_T_FLOAT;
    anon_0::f(field_ptr!(d.as_pointer(), payload)).write(1.5_f64);
    assert!(((((anon_0::f(field_ptr!(d.as_pointer(), payload)).read()) == 1.5_f64) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let e: Value<Slot> = <Value<Slot>>::default();
    (*e.borrow_mut()).tag = Tag_enum_T_REF;
    anon_0::handle(field_ptr!(e.as_pointer(), payload))
        .write(((x.as_pointer()) as Ptr<i32>).to_any());
    assert!(
        ((({ (anon_0::handle(field_ptr!(e.as_pointer(), payload)).read()) } == {
            ((x.as_pointer()) as Ptr<i32>).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
