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
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    #[offset(0)]
    pub items: Ptr<Ptr<u8>>,
    #[offset(8)]
    pub count: i64,
    #[offset(16)]
    pub cursor: i64,
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.items.to_bytes(&mut buf[0..8]);
        self.count.to_bytes(&mut buf[8..16]);
        self.cursor.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: <Ptr<Ptr<u8>>>::from_bytes(&buf[0..8]),
            count: <i64>::from_bytes(&buf[8..16]),
            cursor: <i64>::from_bytes(&buf[16..24]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
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
impl ByteRepr for anon_2 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.lo.to_bytes(&mut buf[0..4]);
        self.hi.to_bytes(&mut buf[4..8]);
        self.curr.to_bytes(&mut buf[8..12]);
        self.step.to_bytes(&mut buf[12..13]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lo: <i32>::from_bytes(&buf[0..4]),
            hi: <i32>::from_bytes(&buf[4..8]),
            curr: <i32>::from_bytes(&buf[8..12]),
            step: <u8>::from_bytes(&buf[12..13]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
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
impl ByteRepr for anon_3 {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.lo.to_bytes(&mut buf[0..8]);
        self.hi.to_bytes(&mut buf[8..16]);
        self.curr.to_bytes(&mut buf[16..24]);
        self.step.to_bytes(&mut buf[24..32]);
        self.width.to_bytes(&mut buf[32..36]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lo: <i64>::from_bytes(&buf[0..8]),
            hi: <i64>::from_bytes(&buf[8..16]),
            curr: <i64>::from_bytes(&buf[16..24]),
            step: <i64>::from_bytes(&buf[24..32]),
            width: <i32>::from_bytes(&buf[32..36]),
        }
    }
}
pub struct anon_0 {
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 40]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        40
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
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Branch {
    #[offset(0)]
    pub choice: Choice_enum,
    #[offset(4)]
    pub index: i32,
    #[offset(8)]
    pub v: anon_0,
}
impl ByteRepr for Branch {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.choice.to_bytes(&mut buf[0..4]);
        self.index.to_bytes(&mut buf[4..8]);
        self.v.to_bytes(&mut buf[8..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            choice: <Choice_enum>::from_bytes(&buf[0..4]),
            index: <i32>::from_bytes(&buf[4..8]),
            v: <anon_0>::from_bytes(&buf[8..48]),
        }
    }
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
    (*p_list.borrow()).v.list().with_mut(|__s: &mut anon_1| {
        __s.items = (items_4.with(|v| v.as_pointer()) as Ptr<Ptr<u8>>)
    });
    (*p_list.borrow())
        .v
        .list()
        .with_mut(|__s: &mut anon_1| __s.count = 3_i64);
    (*p_list.borrow())
        .v
        .list()
        .with_mut(|__s: &mut anon_1| __s.cursor = 1_i64);
    assert!(((((*p_list.borrow()).v.list().with(|__s: &anon_1| __s.count) == 3_i64) as i32) != 0));
    assert!(
        (((((((*p_list.borrow())
            .v
            .list()
            .with(|__s: &anon_1| (__s.items).clone())
            .offset((1) as isize)
            .read())
        .offset((0) as isize)
        .read()) as i32)
            == ('b' as i32)) as i32)
            != 0)
    );
    let p_letters: Value<Branch> = <Value<Branch>>::default();
    (*p_letters.borrow_mut()).choice = Choice_enum_C_LETTERS;
    (*p_letters.borrow_mut()).index = 1;
    (*p_letters.borrow())
        .v
        .letters()
        .with_mut(|__s: &mut anon_2| __s.lo = ('a' as i32));
    (*p_letters.borrow())
        .v
        .letters()
        .with_mut(|__s: &mut anon_2| __s.hi = ('z' as i32));
    (*p_letters.borrow())
        .v
        .letters()
        .with_mut(|__s: &mut anon_2| __s.curr = ('m' as i32));
    (*p_letters.borrow())
        .v
        .letters()
        .with_mut(|__s: &mut anon_2| __s.step = 1_u8);
    assert!(
        (((((*p_letters.borrow())
            .v
            .letters()
            .with(|__s: &anon_2| __s.hi)
            - (*p_letters.borrow())
                .v
                .letters()
                .with(|__s: &anon_2| __s.lo))
            == 25) as i32)
            != 0)
    );
    let p_integers: Value<Branch> = <Value<Branch>>::default();
    (*p_integers.borrow_mut()).choice = Choice_enum_C_INTEGERS;
    (*p_integers.borrow_mut()).index = 2;
    (*p_integers.borrow())
        .v
        .integers()
        .with_mut(|__s: &mut anon_3| __s.lo = 1_i64);
    (*p_integers.borrow())
        .v
        .integers()
        .with_mut(|__s: &mut anon_3| __s.hi = 100_i64);
    (*p_integers.borrow())
        .v
        .integers()
        .with_mut(|__s: &mut anon_3| __s.curr = 1_i64);
    (*p_integers.borrow())
        .v
        .integers()
        .with_mut(|__s: &mut anon_3| __s.step = 1_i64);
    (*p_integers.borrow())
        .v
        .integers()
        .with_mut(|__s: &mut anon_3| __s.width = 3);
    assert!(
        ((((*p_integers.borrow())
            .v
            .integers()
            .with(|__s: &anon_3| __s.hi)
            == 100_i64) as i32)
            != 0)
    );
    assert!(
        ((((*p_integers.borrow())
            .v
            .integers()
            .with(|__s: &anon_3| __s.width)
            == 3) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
