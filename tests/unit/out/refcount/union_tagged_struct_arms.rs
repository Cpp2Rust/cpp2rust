extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Choice_enum = u32;
pub const Choice_enum_C_LIST: Choice_enum = 1;
pub const Choice_enum_C_LETTERS: Choice_enum = 2;
pub const Choice_enum_C_INTEGERS: Choice_enum = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub items: Ptr<Ptr<u8>>,
    #[offset(8)]
    pub count: i64,
    #[offset(16)]
    pub cursor: i64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct anon_2 {
    #[offset(0)]
    pub lo: i32,
    #[offset(4)]
    pub hi: i32,
    #[offset(8)]
    pub curr: i32,
    #[offset(12)]
    pub step: u8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(40)]
pub struct anon_3 {
    #[offset(0)]
    pub lo: i64,
    #[offset(8)]
    pub hi: i64,
    #[offset(16)]
    pub curr: i64,
    #[offset(24)]
    pub step: i64,
    #[offset(32)]
    pub width: i32,
}
#[derive(ByteRepr, DeepClone)]
#[byte_size(40)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(40)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn list(&self) -> Ptr<anon_1> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn letters(&self) -> Ptr<anon_2> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn integers(&self) -> Ptr<anon_3> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 40]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct Branch {
    #[offset(0)]
    pub choice: Choice_enum,
    #[offset(4)]
    pub index: i32,
    #[offset(8)]
    #[byte_size(40)]
    pub v: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    thread_local!(
        static items_4: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
            Ptr::<u8>::from_string_literal(b"a"),
            Ptr::<u8>::from_string_literal(b"b"),
            Ptr::<u8>::from_string_literal(b"c"),
        ])));
    );
    let p_list: Value<Branch> = <Value<Branch>>::default();
    (*p_list.borrow_mut()).choice = Choice_enum_C_LIST;
    (*p_list.borrow_mut()).index = 0;
    field!((*p_list.borrow_mut()).v.list(), items)
        .write((items_4.with(|v| v.as_pointer()) as Ptr<Ptr<u8>>));
    field!((*p_list.borrow_mut()).v.list(), count).write(3_i64);
    field!((*p_list.borrow_mut()).v.list(), cursor).write(1_i64);
    assert!(((((*p_list.borrow()).v.list().with(|__s| __s.count) == 3_i64) as i32) != 0));
    assert!(
        (((((elem!(
            (elem!((*p_list.borrow()).v.list().with(|__s| __s.items.clone()), 1).read()),
            0
        )
        .read()) as i32)
            == ('b' as i32)) as i32)
            != 0)
    );
    let p_letters: Value<Branch> = <Value<Branch>>::default();
    (*p_letters.borrow_mut()).choice = Choice_enum_C_LETTERS;
    (*p_letters.borrow_mut()).index = 1;
    field!((*p_letters.borrow_mut()).v.letters(), lo).write(('a' as i32));
    field!((*p_letters.borrow_mut()).v.letters(), hi).write(('z' as i32));
    field!((*p_letters.borrow_mut()).v.letters(), curr).write(('m' as i32));
    field!((*p_letters.borrow_mut()).v.letters(), step).write(1_u8);
    assert!(
        (((((*p_letters.borrow()).v.letters().with(|__s| __s.hi)
            - (*p_letters.borrow()).v.letters().with(|__s| __s.lo))
            == 25) as i32)
            != 0)
    );
    let p_integers: Value<Branch> = <Value<Branch>>::default();
    (*p_integers.borrow_mut()).choice = Choice_enum_C_INTEGERS;
    (*p_integers.borrow_mut()).index = 2;
    field!((*p_integers.borrow_mut()).v.integers(), lo).write(1_i64);
    field!((*p_integers.borrow_mut()).v.integers(), hi).write(100_i64);
    field!((*p_integers.borrow_mut()).v.integers(), curr).write(1_i64);
    field!((*p_integers.borrow_mut()).v.integers(), step).write(1_i64);
    field!((*p_integers.borrow_mut()).v.integers(), width).write(3);
    assert!(((((*p_integers.borrow()).v.integers().with(|__s| __s.hi) == 100_i64) as i32) != 0));
    assert!(((((*p_integers.borrow()).v.integers().with(|__s| __s.width) == 3) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
