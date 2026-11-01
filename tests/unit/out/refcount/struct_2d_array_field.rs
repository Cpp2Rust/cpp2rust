extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct table {
    #[offset(0)]
    #[byte_size(30)]
    pub rows: Box<[Box<[i8]>]>,
    #[offset(32)]
    pub count: usize,
}
impl Default for table {
    fn default() -> Self {
        {
            table {
                rows: (0..3)
                    .map(|_| (0..10).map(|_| 0_i8).collect::<Box<[i8]>>())
                    .collect::<Box<[Box<[i8]>]>>(),
                count: 0_usize,
            }
        }
    }
}
thread_local!(
    pub static T1_0: Value<table> = Rc::new(RefCell::new(table {
        rows: Box::new([
            i8::array_from_literal(b"alpha\0\0\0\0\0"),
            Box::new([0; 10]),
            Box::new([0; 10]),
        ]),
        count: 1_usize,
    }));
);
thread_local!(
    pub static T2_1: Value<table> = Rc::new(RefCell::new(table {
        rows: Box::new([
            i8::array_from_literal(b"alpha\0\0\0\0\0"),
            i8::array_from_literal(b"beta\0\0\0\0\0\0"),
            Box::new([0; 10]),
        ]),
        count: 2_usize,
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ ((({ (*T1_0.with(Value::clone).borrow()).count } == 1_usize) as i32) != 0) }));
    assert!(
        ((({
            let mut __it1 = (((array_field_ptr!(T1_0.with(|v| v.as_pointer()), rows)
                as Ptr<Box<[i8]>>)
                .offset(0)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"alpha").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ({
            (((((*T1_0.with(Value::clone).borrow()).rows[(1) as usize][(0) as usize] as i32)
                == ('\0' as i32)) as i32)
                != 0)
        })
    );
    assert!(({ ((({ (*T2_1.with(Value::clone).borrow()).count } == 2_usize) as i32) != 0) }));
    assert!(
        ((({
            let mut __it1 = (((array_field_ptr!(T2_1.with(|v| v.as_pointer()), rows)
                as Ptr<Box<[i8]>>)
                .offset(0)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"alpha").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
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
            let mut __it1 = (((array_field_ptr!(T2_1.with(|v| v.as_pointer()), rows)
                as Ptr<Box<[i8]>>)
                .offset(1)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"beta").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ({
            (((((*T2_1.with(Value::clone).borrow()).rows[(2) as usize][(0) as usize] as i32)
                == ('\0' as i32)) as i32)
                != 0)
        })
    );
    let local: Value<table> = Rc::new(RefCell::new(table {
        rows: Box::new([
            i8::array_from_literal(b"one\0\0\0\0\0\0\0"),
            i8::array_from_literal(b"two\0\0\0\0\0\0\0"),
            i8::array_from_literal(b"three\0\0\0\0\0"),
        ]),
        count: 3_usize,
    }));
    assert!(
        ((({
            let mut __it1 = (((array_field_ptr!(local.as_pointer(), rows) as Ptr<Box<[i8]>>)
                .offset(2)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"three").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    (*local.borrow_mut()).rows[(1) as usize][(0) as usize] = (('T' as i32) as i8);
    assert!(
        ((({
            let mut __it1 = (((array_field_ptr!(local.as_pointer(), rows) as Ptr<Box<[i8]>>)
                .offset(1)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"Two").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
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
            let mut __it1 = (((array_field_ptr!(local.as_pointer(), rows) as Ptr<Box<[i8]>>)
                .offset(0)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"one").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    let mut p: Ptr<i8> = (((array_field_ptr!(local.as_pointer(), rows) as Ptr<Box<[i8]>>)
        .offset(2)
        .as_pointer()) as Ptr<i8>);
    assert!((((((elem!(p, 0).read()) as i32) == ('t' as i32)) as i32) != 0));
    assert!(({ ((({ (*local.borrow()).count } == 3_usize) as i32) != 0) }));
    let copy: Value<table> = <Value<table>>::default();
    match ({ ((array_field_ptr!(copy.as_pointer(), rows)) as Ptr<Box<[i8]>>).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(
                &((array_field_ptr!(local.as_pointer(), rows)) as Ptr<Box<[i8]>>).to_any(),
                ::std::mem::size_of::<[[i8; 10]; 3]>() as usize,
            );
            (__tmp_0).clone()
        }
    };
    assert!(
        (((((({
            let mut __it1 = (((array_field_ptr!(copy.as_pointer(), rows) as Ptr<Box<[i8]>>)
                .offset(1)
                .as_pointer()) as Ptr<i8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"Two").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
            && ((({
                let mut __it1 = (((array_field_ptr!(copy.as_pointer(), rows) as Ptr<Box<[i8]>>)
                    .offset(2)
                    .as_pointer()) as Ptr<i8>)
                    .to_c_string_iterator();
                let mut __it2 = Ptr::<i8>::from_string_literal(b"three").to_c_string_iterator();
                loop {
                    let __c1 = __it1.next();
                    let __c2 = __it2.next();
                    if __c1 != __c2 {
                        break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                    }
                    if __c1.is_none() {
                        break 0;
                    }
                }
            } == 0) as i32)
                != 0)) as i32)
            != 0)
    );
    let grid: Value<Box<[Box<[i8]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([
            (('a' as i32) as i8),
            (('b' as i32) as i8),
            (('c' as i32) as i8),
        ]),
        Box::new([
            (('d' as i32) as i8),
            (('e' as i32) as i8),
            (('f' as i32) as i8),
        ]),
    ])));
    let other: Value<Box<[Box<[i8]>]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| (0..3).map(|_| 0_i8).collect::<Box<[i8]>>())
            .collect::<Box<[Box<[i8]>]>>(),
    ));
    match ({ ((other.as_pointer()) as Ptr<Box<[i8]>>).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(
                &((grid.as_pointer()) as Ptr<Box<[i8]>>).to_any(),
                ::std::mem::size_of::<[[i8; 3]; 2]>() as usize,
            );
            (__tmp_1).clone()
        }
    };
    assert!(
        ({
            ((((((((*other.borrow())[(1) as usize][(2) as usize] as i32) == ('f' as i32)) as i32)
                != 0)
                && (((((*other.borrow())[(0) as usize][(1) as usize] as i32) == ('b' as i32))
                    as i32)
                    != 0)) as i32)
                != 0)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = T1_0.with(|_| ());
    let _ = T2_1.with(|_| ());
}
