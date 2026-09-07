extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &Ptr::from_string_literal(b"/tmp/cpp2rust_uafc_test.tmp").to_rust_string(),
            &Ptr::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!(!(*fp.borrow()).is_null());
    {
        let __r = (*fp.borrow()).with(|__f| __f.close());
        (*fp.borrow()).delete();
        __r
    };
    return if ((({
        let __c = ('x' as i32) as u8;
        match (*fp.borrow()).with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    } == ('x' as i32)) as i32)
        != 0)
    {
        1
    } else {
        0
    };
}
