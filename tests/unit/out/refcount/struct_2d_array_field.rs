extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct table {
    #[offset(0)]
    pub rows: Value<Box<[Value<Box<[u8]>>]>>,
    #[offset(32)]
    pub count: usize,
}
impl Clone for table {
    fn clone(&self) -> Self {
        Self {
            rows: Rc::new(RefCell::new((*self.rows.borrow()).clone())),
            count: self.count.clone(),
        }
    }
}
impl Default for table {
    fn default() -> Self {
        table {
            rows: Rc::new(RefCell::new(
                (0..3)
                    .map(|_| Rc::new(RefCell::new((0..10).map(|_| 0_u8).collect::<Box<[u8]>>())))
                    .collect::<Box<[Value<Box<[u8]>>]>>(),
            )),
            count: 0_usize,
        }
    }
}
impl ByteRepr for table {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.rows.borrow()).to_bytes(&mut buf[0..30]);
        self.count.to_bytes(&mut buf[32..40]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            rows: Rc::new(RefCell::new(<Box<[Value<Box<[u8]>>]>>::from_bytes(
                &buf[0..30],
            ))),
            count: <usize>::from_bytes(&buf[32..40]),
        }
    }
}
thread_local!(
    pub static T1_0: Value<table> = Rc::new(RefCell::new(table {
        rows: Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::from(*b"alpha\0\0\0\0\0"))),
            Rc::new(RefCell::new(Box::new([0; 10]))),
            Rc::new(RefCell::new(Box::new([0; 10]))),
        ]))),
        count: 1_usize,
    }));
);
thread_local!(
    pub static T2_1: Value<table> = Rc::new(RefCell::new(table {
        rows: Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::from(*b"alpha\0\0\0\0\0"))),
            Rc::new(RefCell::new(Box::from(*b"beta\0\0\0\0\0\0"))),
            Rc::new(RefCell::new(Box::new([0; 10]))),
        ]))),
        count: 2_usize,
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((({ (*T1_0.with(Value::clone).borrow()).count } == 1_usize) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = ((({ (*T1_0.with(Value::clone).borrow()).rows.clone() }.as_pointer()
                as Ptr<Value<Box<[u8]>>>)
                .offset(0)
                .read()
                .as_pointer()) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"alpha").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((((*{ (*T1_0.with(Value::clone).borrow()).rows.clone() }.borrow())[(1) as usize].borrow()
            [(0) as usize] as i32)
            == ('\0' as i32)) as i32)
            != 0)
    );
    assert!(((({ (*T2_1.with(Value::clone).borrow()).count } == 2_usize) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = ((({ (*T2_1.with(Value::clone).borrow()).rows.clone() }.as_pointer()
                as Ptr<Value<Box<[u8]>>>)
                .offset(0)
                .read()
                .as_pointer()) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"alpha").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = ((({ (*T2_1.with(Value::clone).borrow()).rows.clone() }.as_pointer()
                as Ptr<Value<Box<[u8]>>>)
                .offset(1)
                .read()
                .as_pointer()) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"beta").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((((*{ (*T2_1.with(Value::clone).borrow()).rows.clone() }.borrow())[(2) as usize].borrow()
            [(0) as usize] as i32)
            == ('\0' as i32)) as i32)
            != 0)
    );
    let local: Value<table> = Rc::new(RefCell::new(table {
        rows: Rc::new(RefCell::new(Box::new([
            Rc::new(RefCell::new(Box::from(*b"one\0\0\0\0\0\0\0"))),
            Rc::new(RefCell::new(Box::from(*b"two\0\0\0\0\0\0\0"))),
            Rc::new(RefCell::new(Box::from(*b"three\0\0\0\0\0"))),
        ]))),
        count: 3_usize,
    }));
    assert!(
        ((({
            let mut __it1 = ((({ (*local.borrow()).rows.clone() }.as_pointer()
                as Ptr<Value<Box<[u8]>>>)
                .offset(2)
                .read()
                .as_pointer()) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"three").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    (*{ (*local.borrow()).rows.clone() }.borrow())[(1) as usize].borrow_mut()[(0) as usize] =
        (('T' as i32) as u8);
    assert!(
        ((({
            let mut __it1 = ((({ (*local.borrow()).rows.clone() }.as_pointer()
                as Ptr<Value<Box<[u8]>>>)
                .offset(1)
                .read()
                .as_pointer()) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"Two").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = ((({ (*local.borrow()).rows.clone() }.as_pointer()
                as Ptr<Value<Box<[u8]>>>)
                .offset(0)
                .read()
                .as_pointer()) as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"one").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((({ (*local.borrow()).rows.clone() }.as_pointer() as Ptr<Value<Box<[u8]>>>)
            .offset(2)
            .read()
            .as_pointer()) as Ptr<u8>),
    ));
    assert!(((((((*p.borrow()).offset((0) as isize).read()) as i32) == ('t' as i32)) as i32) != 0));
    assert!(((({ (*local.borrow()).count } == 3_usize) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = T1_0.with(|_| ());
    let _ = T2_1.with(|_| ());
}
