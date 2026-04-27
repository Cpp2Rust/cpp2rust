extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Tag {
    #[default]
    T_NUM_S = 0,
    T_NUM_U = 1,
    T_TEXT = 2,
    T_FLOAT = 3,
    T_REF = 4,
}
#[derive(Default)]
pub union Slot_anon_0 {
    pub text: Value<Ptr<u8>>,
    pub handle: Value<AnyPtr>,
    pub signed_n: Value<i64>,
    pub unsigned_n: Value<u64>,
    pub f: Value<f64>,
}
impl Default for Slot_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Slot_anon_0 {}
#[derive(Default)]
pub struct Slot {
    pub tag: Value<Tag>,
    pub payload: Value<Slot_anon_0>,
}
impl ByteRepr for Slot {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Slot> = <Value<Slot>>::default();
    (*(*a.borrow()).tag.borrow_mut()) = (Tag::T_NUM_S as Tag);
    (*(*(*a.borrow()).payload.borrow()).signed_n.borrow_mut()) = (-7_i32 as i64);
    assert!(((*(*(*a.borrow()).payload.borrow()).signed_n.borrow()) == (-7_i32 as i64)));
    let b: Value<Slot> = <Value<Slot>>::default();
    (*(*b.borrow()).tag.borrow_mut()) = (Tag::T_NUM_U as Tag);
    (*(*(*b.borrow()).payload.borrow()).unsigned_n.borrow_mut()) = 3735928559_u64;
    assert!(((*(*(*b.borrow()).payload.borrow()).unsigned_n.borrow()) == 3735928559_u64));
    let c: Value<Slot> = <Value<Slot>>::default();
    (*(*c.borrow()).tag.borrow_mut()) = (Tag::T_TEXT as Tag);
    (*(*(*c.borrow()).payload.borrow()).text.borrow_mut()) =
        (Ptr::from_string_literal("hello")).clone();
    assert!(
        ((((*(*(*c.borrow()).payload.borrow()).text.borrow())
            .offset((0) as isize)
            .read()) as i32)
            == ('h' as i32))
    );
    let d: Value<Slot> = <Value<Slot>>::default();
    (*(*d.borrow()).tag.borrow_mut()) = (Tag::T_FLOAT as Tag);
    (*(*(*d.borrow()).payload.borrow()).f.borrow_mut()) = 1.5E+0;
    assert!(((*(*(*d.borrow()).payload.borrow()).f.borrow()) == 1.5E+0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let e: Value<Slot> = <Value<Slot>>::default();
    (*(*e.borrow()).tag.borrow_mut()) = (Tag::T_REF as Tag);
    (*(*(*e.borrow()).payload.borrow()).handle.borrow_mut()) =
        ((x.as_pointer()) as Ptr<i32>).to_any();
    assert!({
        let _lhs = (*(*(*e.borrow()).payload.borrow()).handle.borrow()).clone();
        _lhs == ((x.as_pointer()) as Ptr<i32>).to_any()
    });
    return 0;
}
