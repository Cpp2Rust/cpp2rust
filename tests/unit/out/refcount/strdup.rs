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
    let mut d: Ptr<i8> = libcc2rs::strdup_refcount(Ptr::<i8>::from_string_literal(b"hello"));
    assert!((((!((d).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = d.to_c_string_iterator();
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
    libcc2rs::free_refcount((d).to_any());
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"world");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as i8),
        (('b' as i32) as i8),
        (('c' as i32) as i8),
        (('\0' as i32) as i8),
    ])));
    let mut d2: Ptr<i8> = libcc2rs::strdup_refcount((p).clone());
    assert!((((!((d2).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = d2.to_c_string_iterator();
            let mut __it2 = p.to_c_string_iterator();
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
    libcc2rs::free_refcount((d2).to_any());
    let mut d3: Ptr<i8> = libcc2rs::strdup_refcount((buf.as_pointer() as Ptr<i8>));
    assert!((((!((d3).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = d3.to_c_string_iterator();
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
    libcc2rs::free_refcount((d3).to_any());
    let mut d4: Ptr<i8> = Ptr::<i8>::null();
    d4 = libcc2rs::strdup_refcount((p).clone());
    assert!((((!((d4).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = d4.to_c_string_iterator();
            let mut __it2 = p.to_c_string_iterator();
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
    libcc2rs::free_refcount((d4).to_any());
    let rec: Value<record> = Rc::new(RefCell::new(record {
        name: Ptr::<i8>::null(),
    }));
    let mut r: Ptr<record> = (rec.as_pointer());
    let __rhs = libcc2rs::strdup_refcount((p).clone());
    field!(r, name).write(__rhs);
    assert!((((!((r.with(|__s| __s.name.clone())).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = r.with(|__s| __s.name.clone()).to_c_string_iterator();
            let mut __it2 = p.to_c_string_iterator();
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
    libcc2rs::free_refcount((r.with(|__s| __s.name.clone()) as Ptr<i8>).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
