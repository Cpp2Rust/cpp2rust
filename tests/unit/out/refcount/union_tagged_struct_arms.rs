extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Choice_enum {
    #[default]
    C_LIST = 1,
    C_LETTERS = 2,
    C_INTEGERS = 3,
}
impl From<i32> for Choice_enum {
    fn from(n: i32) -> Choice_enum {
        match n {
            1 => Choice_enum::C_LIST,
            2 => Choice_enum::C_LETTERS,
            3 => Choice_enum::C_INTEGERS,
            _ => panic!("invalid Choice_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Choice_enum);
#[derive(Default)]
pub struct anon_1 {
    pub items: Value<Ptr<Ptr<u8>>>,
    pub count: Value<i64>,
    pub cursor: Value<i64>,
}
impl ByteRepr for anon_1 {}
#[derive(Default)]
pub struct anon_2 {
    pub lo: Value<i32>,
    pub hi: Value<i32>,
    pub curr: Value<i32>,
    pub step: Value<u8>,
}
impl ByteRepr for anon_2 {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.lo.borrow()).to_bytes(&mut buf[0..4]);
        (*self.hi.borrow()).to_bytes(&mut buf[4..8]);
        (*self.curr.borrow()).to_bytes(&mut buf[8..12]);
        (*self.step.borrow()).to_bytes(&mut buf[12..13]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lo: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            hi: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            curr: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
            step: Rc::new(RefCell::new(<u8>::from_bytes(&buf[12..13]))),
        }
    }
}
#[derive(Default)]
pub struct anon_3 {
    pub lo: Value<i64>,
    pub hi: Value<i64>,
    pub curr: Value<i64>,
    pub step: Value<i64>,
    pub width: Value<i32>,
}
impl ByteRepr for anon_3 {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.lo.borrow()).to_bytes(&mut buf[0..8]);
        (*self.hi.borrow()).to_bytes(&mut buf[8..16]);
        (*self.curr.borrow()).to_bytes(&mut buf[16..24]);
        (*self.step.borrow()).to_bytes(&mut buf[24..32]);
        (*self.width.borrow()).to_bytes(&mut buf[32..36]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lo: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            hi: Rc::new(RefCell::new(<i64>::from_bytes(&buf[8..16]))),
            curr: Rc::new(RefCell::new(<i64>::from_bytes(&buf[16..24]))),
            step: Rc::new(RefCell::new(<i64>::from_bytes(&buf[24..32]))),
            width: Rc::new(RefCell::new(<i32>::from_bytes(&buf[32..36]))),
        }
    }
}
#[derive(Clone)]
pub struct anon_0 {
    __store: libcc2rs::UnionStorage,
}
impl anon_0 {
    pub fn list(&self) -> Ptr<anon_1> {
        self.__store.reinterpret(0)
    }
    pub fn letters(&self) -> Ptr<anon_2> {
        self.__store.reinterpret(0)
    }
    pub fn integers(&self) -> Ptr<anon_3> {
        self.__store.reinterpret(0)
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __store: libcc2rs::UnionStorage::new(40),
        }
    }
}
impl ByteRepr for anon_0 {}
#[derive(Default)]
pub struct Branch {
    pub choice: Value<Choice_enum>,
    pub index: Value<i32>,
    pub v: Value<anon_0>,
}
impl ByteRepr for Branch {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    thread_local!(
        static items_4: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
            Ptr::from_string_literal(b"a"),
            Ptr::from_string_literal(b"b"),
            Ptr::from_string_literal(b"c"),
        ])));
    );
    let p_list: Value<Branch> = <Value<Branch>>::default();
    (*(*p_list.borrow()).choice.borrow_mut()) = Choice_enum::C_LIST;
    (*(*p_list.borrow()).index.borrow_mut()) = 0;
    (*(*(*(*p_list.borrow()).v.borrow()).list().upgrade().deref())
        .items
        .borrow_mut()) = (items_4.with(Value::clone).as_pointer() as Ptr<Ptr<u8>>);
    (*(*(*(*p_list.borrow()).v.borrow()).list().upgrade().deref())
        .count
        .borrow_mut()) = 3_i64;
    (*(*(*(*p_list.borrow()).v.borrow()).list().upgrade().deref())
        .cursor
        .borrow_mut()) = 1_i64;
    assert!(
        ((((*(*(*(*p_list.borrow()).v.borrow()).list().upgrade().deref())
            .count
            .borrow())
            == 3_i64) as i32)
            != 0)
    );
    assert!(
        (((((((*(*(*(*p_list.borrow()).v.borrow()).list().upgrade().deref())
            .items
            .borrow())
        .offset((1) as isize)
        .read())
        .offset((0) as isize)
        .read()) as i32)
            == ('b' as i32)) as i32)
            != 0)
    );
    let p_letters: Value<Branch> = <Value<Branch>>::default();
    (*(*p_letters.borrow()).choice.borrow_mut()) = Choice_enum::C_LETTERS;
    (*(*p_letters.borrow()).index.borrow_mut()) = 1;
    (*(*(*(*p_letters.borrow()).v.borrow())
        .letters()
        .upgrade()
        .deref())
    .lo
    .borrow_mut()) = ('a' as i32);
    (*(*(*(*p_letters.borrow()).v.borrow())
        .letters()
        .upgrade()
        .deref())
    .hi
    .borrow_mut()) = ('z' as i32);
    (*(*(*(*p_letters.borrow()).v.borrow())
        .letters()
        .upgrade()
        .deref())
    .curr
    .borrow_mut()) = ('m' as i32);
    (*(*(*(*p_letters.borrow()).v.borrow())
        .letters()
        .upgrade()
        .deref())
    .step
    .borrow_mut()) = 1_u8;
    assert!(
        (((((*(*(*(*p_letters.borrow()).v.borrow())
            .letters()
            .upgrade()
            .deref())
        .hi
        .borrow())
            - (*(*(*(*p_letters.borrow()).v.borrow())
                .letters()
                .upgrade()
                .deref())
            .lo
            .borrow()))
            == 25) as i32)
            != 0)
    );
    let p_integers: Value<Branch> = <Value<Branch>>::default();
    (*(*p_integers.borrow()).choice.borrow_mut()) = Choice_enum::C_INTEGERS;
    (*(*p_integers.borrow()).index.borrow_mut()) = 2;
    (*(*(*(*p_integers.borrow()).v.borrow())
        .integers()
        .upgrade()
        .deref())
    .lo
    .borrow_mut()) = 1_i64;
    (*(*(*(*p_integers.borrow()).v.borrow())
        .integers()
        .upgrade()
        .deref())
    .hi
    .borrow_mut()) = 100_i64;
    (*(*(*(*p_integers.borrow()).v.borrow())
        .integers()
        .upgrade()
        .deref())
    .curr
    .borrow_mut()) = 1_i64;
    (*(*(*(*p_integers.borrow()).v.borrow())
        .integers()
        .upgrade()
        .deref())
    .step
    .borrow_mut()) = 1_i64;
    (*(*(*(*p_integers.borrow()).v.borrow())
        .integers()
        .upgrade()
        .deref())
    .width
    .borrow_mut()) = 3;
    assert!(
        ((((*(*(*(*p_integers.borrow()).v.borrow())
            .integers()
            .upgrade()
            .deref())
        .hi
        .borrow())
            == 100_i64) as i32)
            != 0)
    );
    assert!(
        ((((*(*(*(*p_integers.borrow()).v.borrow())
            .integers()
            .upgrade()
            .deref())
        .width
        .borrow())
            == 3) as i32)
            != 0)
    );
    return 0;
}
