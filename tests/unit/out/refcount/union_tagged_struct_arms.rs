extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Choice {
    #[default]
    C_LIST = 1,
    C_LETTERS = 2,
    C_INTEGERS = 3,
}
#[derive(Default)]
pub struct Branch_anon_0_anon_0 {
    pub items: Value<Ptr<Ptr<u8>>>,
    pub count: Value<i64>,
    pub cursor: Value<i64>,
}
impl ByteRepr for Branch_anon_0_anon_0 {}
#[derive(Default)]
pub struct Branch_anon_0_anon_1 {
    pub lo: Value<i32>,
    pub hi: Value<i32>,
    pub curr: Value<i32>,
    pub step: Value<u8>,
}
impl ByteRepr for Branch_anon_0_anon_1 {}
#[derive(Default)]
pub struct Branch_anon_0_anon_2 {
    pub lo: Value<i64>,
    pub hi: Value<i64>,
    pub curr: Value<i64>,
    pub step: Value<i64>,
    pub width: Value<i32>,
}
impl ByteRepr for Branch_anon_0_anon_2 {}
#[derive(Default)]
pub struct Branch_anon_0 {
    pub list: Value<Branch_anon_0_anon_0>,
    pub letters: Value<Branch_anon_0_anon_1>,
    pub integers: Value<Branch_anon_0_anon_2>,
}
impl ByteRepr for Branch_anon_0 {}
#[derive(Default)]
pub struct Branch {
    pub choice: Value<Choice>,
    pub index: Value<i32>,
    pub v: Value<Branch_anon_0>,
}
impl ByteRepr for Branch {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    thread_local!(
        static items: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
            Ptr::from_string_literal("a"),
            Ptr::from_string_literal("b"),
            Ptr::from_string_literal("c"),
        ])));
    );
    let p_list: Value<Branch> = <Value<Branch>>::default();
    (*(*p_list.borrow()).choice.borrow_mut()) = (Choice::C_LIST as Choice);
    (*(*p_list.borrow()).index.borrow_mut()) = 0;
    (*(*(*(*p_list.borrow()).v.borrow()).list.borrow())
        .items
        .borrow_mut()) = (items.with(Value::clone).as_pointer() as Ptr<Ptr<u8>>);
    (*(*(*(*p_list.borrow()).v.borrow()).list.borrow())
        .count
        .borrow_mut()) = 3_i64;
    (*(*(*(*p_list.borrow()).v.borrow()).list.borrow())
        .cursor
        .borrow_mut()) = 1_i64;
    assert!(
        ((*(*(*(*p_list.borrow()).v.borrow()).list.borrow())
            .count
            .borrow())
            == 3_i64)
    );
    assert!(
        (((((*(*(*(*p_list.borrow()).v.borrow()).list.borrow())
            .items
            .borrow())
        .offset((1) as isize)
        .read())
        .offset((0) as isize)
        .read()) as i32)
            == ('b' as i32))
    );
    let p_letters: Value<Branch> = <Value<Branch>>::default();
    (*(*p_letters.borrow()).choice.borrow_mut()) = (Choice::C_LETTERS as Choice);
    (*(*p_letters.borrow()).index.borrow_mut()) = 1;
    (*(*(*(*p_letters.borrow()).v.borrow()).letters.borrow())
        .lo
        .borrow_mut()) = ('a' as i32);
    (*(*(*(*p_letters.borrow()).v.borrow()).letters.borrow())
        .hi
        .borrow_mut()) = ('z' as i32);
    (*(*(*(*p_letters.borrow()).v.borrow()).letters.borrow())
        .curr
        .borrow_mut()) = ('m' as i32);
    (*(*(*(*p_letters.borrow()).v.borrow()).letters.borrow())
        .step
        .borrow_mut()) = 1_u8;
    assert!(
        (((*(*(*(*p_letters.borrow()).v.borrow()).letters.borrow())
            .hi
            .borrow())
            - (*(*(*(*p_letters.borrow()).v.borrow()).letters.borrow())
                .lo
                .borrow()))
            == 25)
    );
    let p_integers: Value<Branch> = <Value<Branch>>::default();
    (*(*p_integers.borrow()).choice.borrow_mut()) = (Choice::C_INTEGERS as Choice);
    (*(*p_integers.borrow()).index.borrow_mut()) = 2;
    (*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
        .lo
        .borrow_mut()) = 1_i64;
    (*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
        .hi
        .borrow_mut()) = 100_i64;
    (*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
        .curr
        .borrow_mut()) = 1_i64;
    (*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
        .step
        .borrow_mut()) = 1_i64;
    (*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
        .width
        .borrow_mut()) = 3;
    assert!(
        ((*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
            .hi
            .borrow())
            == 100_i64)
    );
    assert!(
        ((*(*(*(*p_integers.borrow()).v.borrow()).integers.borrow())
            .width
            .borrow())
            == 3)
    );
    return 0;
}
