extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct record {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<i8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        Ptr::<i8>::from_string_literal(b"hello"),
    )));
    assert!((((!((*d.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*d.borrow()).to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"hello").to_c_string_iterator();
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
    libcc2rs::free_refcount((*d.borrow()).to_any());
    let p: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::from_string_literal(b"world")));
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('c' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    let d2: Value<Ptr<i8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        (*p.borrow()).clone(),
    )));
    assert!((((!((*d2.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*d2.borrow()).to_c_string_iterator();
            let mut __it2 = (*p.borrow()).to_c_string_iterator();
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
    libcc2rs::free_refcount((*d2.borrow()).to_any());
    let d3: Value<Ptr<i8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        (buf.as_pointer() as Ptr<i8>),
    )));
    assert!((((!((*d3.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*d3.borrow()).to_c_string_iterator();
            let mut __it2 = (buf.as_pointer() as Ptr<i8>).to_c_string_iterator();
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
    libcc2rs::free_refcount((*d3.borrow()).to_any());
    let d4: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
    (*d4.borrow_mut()) = libcc2rs::strdup_refcount((*p.borrow()).clone());
    assert!((((!((*d4.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*d4.borrow()).to_c_string_iterator();
            let mut __it2 = (*p.borrow()).to_c_string_iterator();
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
    libcc2rs::free_refcount((*d4.borrow()).to_any());
    let rec: Value<record> = Rc::new(RefCell::new(record {
        name: Ptr::<i8>::null(),
    }));
    let r: Value<Ptr<record>> = Rc::new(RefCell::new((rec.as_pointer())));
    let __rhs = libcc2rs::strdup_refcount((*p.borrow()).clone());
    field!((*r.borrow()), name).write(__rhs);
    assert!((((!(((*r.borrow()).with(|__s| __s.name.clone())).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*r.borrow())
                .with(|__s| __s.name.clone())
                .to_c_string_iterator();
            let mut __it2 = (*p.borrow()).to_c_string_iterator();
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
    libcc2rs::free_refcount(((*r.borrow()).with(|__s| __s.name.clone()) as Ptr<i8>).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
